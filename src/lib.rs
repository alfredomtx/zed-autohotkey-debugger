use std::{env, net::Ipv4Addr, path::Path, sync::OnceLock, time::Duration};

use zed_extension_api::{
    self as zed, download_file, latest_github_release, serde_json, DebugAdapterBinary, DebugConfig,
    DebugRequest, DebugScenario, DebugTaskDefinition, DownloadedFileType, GithubReleaseAsset,
    GithubReleaseOptions, StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    TcpArguments, Worktree,
};

const ADAPTER_NAME: &str = "autohotkey";
const GITHUB_REPO: &str = "helsmy/autohotkey-debug-adapter";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_PORT: u16 = 9005;

fn request_type_from_config(
    config: &serde_json::Value,
) -> Result<StartDebuggingRequestArgumentsRequest, String> {
    match config.get("request").and_then(|v| v.as_str()) {
        Some("launch") => Ok(StartDebuggingRequestArgumentsRequest::Launch),
        Some("attach") => Ok(StartDebuggingRequestArgumentsRequest::Attach),
        Some(other) => Err(format!(
            "Invalid request type '{}', expected 'launch' or 'attach'",
            other
        )),
        None => Ok(StartDebuggingRequestArgumentsRequest::Launch),
    }
}

fn validate_adapter_name(name: &str) -> Result<(), String> {
    if name != ADAPTER_NAME {
        return Err(format!(
            "Unsupported adapter '{}', expected '{}'",
            name, ADAPTER_NAME
        ));
    }
    Ok(())
}

struct AutoHotkeyDebugger {
    cached_version: OnceLock<String>,
}

impl AutoHotkeyDebugger {
    fn adapter_dir(&self) -> String {
        env::current_dir()
            .unwrap()
            .join(ADAPTER_NAME)
            .to_string_lossy()
            .into_owned()
    }

    fn versioned_dir(&self, version: &str) -> String {
        format!("{}/{}_{}", self.adapter_dir(), ADAPTER_NAME, version)
    }

    fn fetch_latest_release() -> Result<(GithubReleaseAsset, String), String> {
        let release = latest_github_release(
            GITHUB_REPO,
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let version = release.version.trim_start_matches('v').to_string();
        let expected_name = format!("autohotkey-debug-{}.vsix", version);

        let asset = release
            .assets
            .into_iter()
            .find(|a| a.name.ends_with(".vsix"))
            .ok_or_else(|| {
                format!(
                    "No .vsix asset found in release (expected {})",
                    expected_name
                )
            })?;

        Ok((asset, version))
    }

    fn ensure_adapter_installed(&mut self) -> Result<String, String> {
        if let Some(version) = self.cached_version.get() {
            return Ok(version.clone());
        }

        match Self::fetch_latest_release() {
            Ok((asset, version)) => {
                let versioned_dir = self.versioned_dir(&version);

                if !Path::new(&versioned_dir).exists() {
                    let adapter_dir = self.adapter_dir();
                    std::fs::remove_dir_all(&adapter_dir).ok();
                    std::fs::create_dir_all(&adapter_dir)
                        .map_err(|e| format!("Failed to create adapter directory: {}", e))?;

                    download_file(&asset.download_url, &versioned_dir, DownloadedFileType::Zip)?;
                }

                self.cached_version.set(version.clone()).ok();
                Ok(version)
            }
            Err(fetch_err) => {
                let prefix = format!("{}_", ADAPTER_NAME);
                let adapter_dir = self.adapter_dir();

                if let Ok(entries) = std::fs::read_dir(&adapter_dir) {
                    let version = entries
                        .filter_map(|e| e.ok())
                        .filter_map(|entry| {
                            entry
                                .file_name()
                                .to_string_lossy()
                                .strip_prefix(&prefix)
                                .map(ToOwned::to_owned)
                        })
                        .max();

                    if let Some(v) = version {
                        self.cached_version.set(v.clone()).ok();
                        return Ok(v);
                    }
                }

                Err(format!(
                    "Failed to fetch release and no cached version found: {}",
                    fetch_err
                ))
            }
        }
    }

    fn ahk_exe_path(&self, version: &str) -> String {
        Path::new(&self.versioned_dir(version))
            .join("extension/bin/AutoHotkey.exe")
            .to_string_lossy()
            .into_owned()
    }

    fn adapter_script_path(&self, version: &str) -> String {
        Path::new(&self.versioned_dir(version))
            .join("extension/ahkdbg/debugAdapter.ahk")
            .to_string_lossy()
            .into_owned()
    }

    fn build_binary(
        &self,
        version: &str,
        config: DebugTaskDefinition,
        user_provided_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        let ahk_exe = user_provided_path.unwrap_or_else(|| self.ahk_exe_path(version));
        let adapter_script = self.adapter_script_path(version);

        let port = DEFAULT_PORT;

        let connection = TcpArguments {
            port,
            host: Ipv4Addr::LOCALHOST.to_bits(),
            timeout: Some(DEFAULT_TIMEOUT.as_millis() as u64),
        };

        let request = Self::parse_request_kind(&config.config)?;

        Ok(DebugAdapterBinary {
            command: Some(ahk_exe),
            arguments: vec![adapter_script, format!("--port={}", port)],
            envs: vec![],
            cwd: Some(worktree.root_path()),
            connection: Some(connection),
            request_args: StartDebuggingRequestArguments {
                configuration: config.config,
                request,
            },
        })
    }

    fn parse_request_kind(
        config_json: &str,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        let config: serde_json::Value = serde_json::from_str(config_json)
            .map_err(|e| format!("Failed to parse config JSON: {}", e))?;

        request_type_from_config(&config)
    }
}

impl zed::Extension for AutoHotkeyDebugger {
    fn new() -> Self {
        Self {
            cached_version: OnceLock::new(),
        }
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        validate_adapter_name(&adapter_name)?;

        let version = self.ensure_adapter_installed()?;
        self.build_binary(&version, config, user_provided_debug_adapter_path, worktree)
    }

    fn dap_request_kind(
        &mut self,
        adapter_name: String,
        config: serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        validate_adapter_name(&adapter_name)?;

        request_type_from_config(&config)
    }

    fn dap_config_to_scenario(&mut self, config: DebugConfig) -> Result<DebugScenario, String> {
        validate_adapter_name(&config.adapter)?;

        let scenario_config = match &config.request {
            DebugRequest::Launch(launch) => {
                serde_json::json!({
                    "request": "launch",
                    "program": launch.program,
                    "cwd": launch.cwd,
                    "args": launch.args,
                    "stopOnEntry": config.stop_on_entry.unwrap_or(false),
                })
            }
            DebugRequest::Attach(_) => {
                return Err("AutoHotkey debugger does not support attach mode".into());
            }
        };

        Ok(DebugScenario {
            adapter: config.adapter,
            label: config.label,
            build: None,
            config: scenario_config.to_string(),
            tcp_connection: None,
        })
    }
}

zed::register_extension!(AutoHotkeyDebugger);
