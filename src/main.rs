#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![doc = include_str!("../README.md")]

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const DASH_BASE: &str = "https://dash.cloudflare.com";

#[derive(Parser)]
#[command(name = "cfurl")]
#[command(about = "Quick access to Cloudflare dashboard pages", long_about = None)]
#[command(version)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Open DNS settings for a zone
	Dns {
		/// Zone/domain name (e.g., miguel.build)
		zone: String,
	},

	/// Open Workers & Pages dashboard
	Workers {
		/// Specific worker name (optional)
		name: Option<String>,
	},

	/// Open Pages dashboard
	Pages {
		/// Specific pages project name (optional)
		name: Option<String>,
	},

	/// Open R2 object storage
	R2 {
		/// Specific bucket name (optional)
		bucket: Option<String>,
	},

	/// Open D1 databases
	D1 {
		/// Specific database name (optional)
		database: Option<String>,
	},

	/// Open KV namespaces
	Kv {
		/// Specific namespace (optional)
		namespace: Option<String>,
	},

	/// Open zone analytics
	Analytics {
		/// Zone/domain name
		zone: String,
	},

	/// Open security settings (WAF, etc.)
	Security {
		/// Zone/domain name
		zone: String,
		/// Specific section: waf, events, ddos, bots
		#[arg(short, long)]
		section: Option<String>,
	},

	/// Open SSL/TLS settings
	Ssl {
		/// Zone/domain name
		zone: String,
	},

	/// Open caching settings
	Caching {
		/// Zone/domain name
		zone: String,
	},

	/// Open rules settings (redirects, transforms, etc.)
	Rules {
		/// Zone/domain name
		zone: String,
	},

	/// Open speed/optimization settings
	Speed {
		/// Zone/domain name
		zone: String,
	},

	/// Open email routing settings
	Email {
		/// Zone/domain name
		zone: String,
	},

	/// Open Spectrum settings
	Spectrum {
		/// Zone/domain name
		zone: String,
	},

	/// Open network settings
	Network {
		/// Zone/domain name
		zone: String,
	},

	/// Open traffic settings (load balancing, health checks)
	Traffic {
		/// Zone/domain name
		zone: String,
	},

	/// Open scrape shield settings
	Scrape {
		/// Zone/domain name
		zone: String,
	},

	/// Open Zero Trust dashboard
	#[command(alias = "zt")]
	ZeroTrust,

	/// Open Access settings
	Access,

	/// Open Cloudflare Tunnels
	Tunnels,

	/// Open Cloudflare Stream
	Stream,

	/// Open Cloudflare Images
	Images,

	/// Open Queues
	Queues,

	/// Open Workers AI
	Ai,

	/// Open Vectorize
	Vectorize,

	/// Open Hyperdrive
	Hyperdrive,

	/// Open Durable Objects
	#[command(alias = "do")]
	DurableObjects,

	/// Open account settings
	Account,

	/// Open billing page
	Billing,

	/// Open audit log
	#[command(alias = "audit")]
	AuditLog,

	/// Open API tokens page
	#[command(alias = "tokens")]
	ApiTokens,

	/// Open domain registrar
	#[command(alias = "domains")]
	Registrar,

	/// Open Turnstile (CAPTCHA)
	Turnstile,

	/// Open Zaraz
	Zaraz {
		/// Zone/domain name
		zone: String,
	},

	/// Open Web Analytics
	#[command(alias = "wa")]
	WebAnalytics,

	/// Open Logs (Logpush)
	Logs {
		/// Zone/domain name (optional for account-level)
		zone: Option<String>,
	},

	/// Open zone overview
	Zone {
		/// Zone/domain name
		zone: String,
	},

	/// Open the main dashboard
	#[command(alias = "home")]
	Dash,
}

fn main() {
	let cli = Cli::parse();

	let url = match cli.command {
		Commands::Dns { zone } => zone_url(&zone, "dns"),
		Commands::Analytics { zone } => zone_url(&zone, "analytics"),
		Commands::Security { zone, section } => {
			let path = match section.as_deref() {
				Some("waf") => "security/waf",
				Some("events") => "security/events",
				Some("ddos") => "security/ddos",
				Some("bots") => "security/bots",
				_ => "security",
			};
			zone_url(&zone, path)
		},
		Commands::Ssl { zone } => zone_url(&zone, "ssl-tls"),
		Commands::Caching { zone } => zone_url(&zone, "caching"),
		Commands::Rules { zone } => zone_url(&zone, "rules"),
		Commands::Speed { zone } => zone_url(&zone, "speed"),
		Commands::Email { zone } => zone_url(&zone, "email"),
		Commands::Spectrum { zone } => zone_url(&zone, "spectrum"),
		Commands::Network { zone } => zone_url(&zone, "network"),
		Commands::Traffic { zone } => zone_url(&zone, "traffic"),
		Commands::Scrape { zone } => zone_url(&zone, "content-protection"),
		Commands::Zaraz { zone } => zone_url(&zone, "zaraz"),
		Commands::Zone { zone } => zone_url(&zone, ""),
		Commands::Logs { zone: Some(zone) } => zone_url(&zone, "analytics/logs"),

		Commands::Workers { name } => name.map_or_else(
			|| account_url("workers-and-pages"),
			|n| account_url(&format!("workers/services/view/{n}")),
		),
		Commands::Pages { name } => name.map_or_else(
			|| account_url("workers-and-pages"),
			|n| account_url(&format!("pages/view/{n}")),
		),
		Commands::R2 { bucket } => bucket.map_or_else(
			|| account_url("r2"),
			|b| account_url(&format!("r2/default/buckets/{b}")),
		),
		Commands::D1 { database } => database.map_or_else(
			|| account_url("workers/d1"),
			|d| account_url(&format!("workers/d1/databases/{d}")),
		),
		Commands::Kv { namespace } => namespace.map_or_else(
			|| account_url("workers/kv"),
			|n| account_url(&format!("workers/kv/namespaces/{n}")),
		),
		Commands::ZeroTrust | Commands::Access => account_url("access"),
		Commands::Tunnels => account_url("access/tunnels"),
		Commands::Stream => account_url("stream"),
		Commands::Images => account_url("images"),
		Commands::Queues => account_url("queues"),
		Commands::Ai => account_url("ai"),
		Commands::Vectorize => account_url("vectorize"),
		Commands::Hyperdrive => account_url("hyperdrive"),
		Commands::DurableObjects => account_url("workers/durable-objects"),
		Commands::Account => account_url(""),
		Commands::Billing => account_url("billing"),
		Commands::AuditLog => account_url("audit-log"),
		Commands::ApiTokens => format!("{DASH_BASE}/profile/api-tokens"),
		Commands::Registrar => account_url("domains"),
		Commands::Turnstile => account_url("turnstile"),
		Commands::WebAnalytics => account_url("web-analytics"),
		Commands::Logs { zone: None } => account_url("logs"),
		Commands::Dash => DASH_BASE.to_string(),
	};

	let spinner = ProgressBar::new_spinner();
	spinner.set_style(
		ProgressStyle::default_spinner()
			.template("{spinner:.cyan} {msg}")
			.unwrap()
			.tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
	);
	spinner.set_message("Opening in your browser...");
	spinner.enable_steady_tick(Duration::from_millis(80));

	if let Err(e) = open::that(&url) {
		spinner.finish_and_clear();
		eprintln!("✗ Failed to open browser: {e}");
		std::process::exit(1);
	}

	std::thread::sleep(Duration::from_secs(1));
	spinner.finish_and_clear();
	println!("✓ Opened");
}

fn zone_url(zone: &str, path: &str) -> String {
	if path.is_empty() {
		format!("{DASH_BASE}/?to=/:account/{zone}")
	} else {
		format!("{DASH_BASE}/?to=/:account/{zone}/{path}")
	}
}

fn account_url(path: &str) -> String {
	if path.is_empty() {
		format!("{DASH_BASE}/?to=/:account")
	} else {
		format!("{DASH_BASE}/?to=/:account/{path}")
	}
}
