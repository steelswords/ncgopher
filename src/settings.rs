use config::{Config, ConfigError, File, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File as FsFile;
use std::io::Write;
use std::path::Path;
//use cursive::theme::{Theme, BorderStyle};
//use cursive::theme::BaseColor::*;
//use cursive::theme::Color::*;
//use cursive::theme::PaletteColor::*;

pub struct Settings {
    config: Config,
    config_filename: String,
    themes: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::new();
        let mut settings = Settings {
            config: s,
            config_filename: String::new(),
            themes: HashMap::new(),
        };

        // Create config dir if necessary
        match dirs::config_dir() {
            Some(mut dir) => {
                dir.push(env!("CARGO_PKG_NAME"));
                let dir = dir.into_os_string().into_string().unwrap();
                if !Path::new(&dir).exists() {
                    match fs::create_dir_all(dir) {
                        Err(why) => warn!("Could not create config dir: {}", why),
                        Ok(()) => (),
                    }
                }
            }
            None => {
                println!("Could not determine config dir");
            }
        };

        let confdir: String = match dirs::config_dir() {
            Some(mut dir) => {
                dir.push(env!("CARGO_PKG_NAME"));
                dir.push("config.toml");
                dir.into_os_string().into_string().unwrap()
            }
            None => String::new(),
        };
        settings.config_filename = confdir.clone();
        println!("Looking for config file {}", confdir);

        // Set defaults
        settings.config.set_default("download_path", "Downloads")?;
        settings
            .config
            .set_default("homepage", "gopher://jan.bio:70/1/ncgopher/")?;
        settings.config.set_default("debug", false)?;
        settings.config.set_default("theme", "lightmode")?;
        settings.config.set_default("html_command", "")?;
        settings.config.set_default("image_command", "")?;
        settings.config.set_default("telnet_command", "")?;
        settings.config.set_default("textwrap", "80")?;
        settings.themes.insert(
            "darkmode".to_string(),
            include_str!("themes/darkmode.toml").to_string(),
        );
        settings.themes.insert(
            "lightmode".to_string(),
            include_str!("themes/lightmode.toml").to_string(),
        );

        if Path::new(confdir.as_str()).exists() {
            // Start off by merging in the "default" configuration file
            match settings.config.merge(File::with_name(confdir.as_str())) {
                Ok(_) => (),
                Err(e) => {
                    println!("Could not read config file: {}", e);
                }
            }
        }

        // Debug: Now that we're done, let's access our configuration
        //println!("debug: {:?}", settings.config.get_bool("debug").unwrap());
        //println!("homepage: {:?}", settings.config.get::<String>("homepage").unwrap());
        //println!("theme: {:?}", settings.config.get::<String>("theme").unwrap());

        // You can deserialize (and thus freeze) the entire configuration as
        //s.try_into()
        Ok(settings)
    }

    pub fn write_settings_to_file(&mut self) -> std::io::Result<()> {
        let filename = self.config_filename.clone();
        info!("Saving settings to file: {}", filename);
        // Create a path to the desired file
        let path = Path::new(&filename);

        let mut file = match FsFile::create(&path) {
            Err(why) => return Err(why),
            Ok(file) => file,
        };

        if let Err(why) = file.write(b"# Automatically generated by ncgopher.\n") {
            return Err(why);
        };

        let config: HashMap<String, String> =
            match self.config.clone().try_into::<HashMap<String, String>>() {
                Ok(str) => str,
                Err(err) => {
                    warn!("Could not write config: {}", err);
                    HashMap::new()
                }
            };
        let toml = toml::to_string(&config).unwrap();
        file.write_all(toml.as_bytes())
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<&mut Config, ConfigError>
    where
        T: Into<Value>,
    {
        self.config.set::<T>(key, value)
    }

    /*
    pub fn get<'de, T: Deserialize<'de>>(&self, key: &'de str) -> Result<T, ConfigError> {
        self.config.get::<T>(key)
    }
    */

    pub fn get_str(&self, key: &str) -> Result<String, ConfigError> {
        self.config.get_str(key)
    }

    /*
    // Get custom theme. TODO: Read from config file
    pub fn get_theme(&self) -> Theme {
        let mut theme: Theme = Theme::default();
        theme.shadow = true;
        theme.borders = BorderStyle::Simple;
        theme.palette[Background] = Dark(Blue);
        theme.palette[View] = Light(Black);
        theme.palette[Primary] = Dark(Blue);
        theme.palette[Highlight] = Light(Cyan);
        theme.palette[HighlightInactive] = Dark(Cyan);
        theme.palette[TitlePrimary] = Dark(Magenta);
        /*
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    Background,
    Shadow,
    View,
    Primary,
    Secondary,
    Tertiary,
    TitlePrimary,
    TitleSecondary,
    Highlight,
    HighlightInactive,
        */
        theme
    }
    */

    pub fn get_theme_by_name(&self, name: String) -> &str {
        self.themes[&name].as_str()
    }
}
