use std::path::Path;

#[macro_use]
extern crate fstrings;

use console::StyledObject;

pub mod shared;
pub mod subcmd;

// TODO
// investigate rhai vs ts (swc+deno_core) scripting
// clean backups at start of watch
// implement derive + plot
// implement watch over data
// implement watch over scripts
// implement watch over everything?
// implement plot
// verbose arg
// pie chart and map chart
// investigate dialoguer, indicatif family
// investigate long help/arg text vs documentation

fn main() {
    // CONSOLE STYLE MEANING
    // red:     fatal error
    // yellow:  warning
    // green:   operation successful
    // cyan:    file path
    // blue:    directory path
    // magenta: file / script contents
    // bold:    console usage
    // italic:  script usage
    let cso = CommonStyledObjects::new();

    let config_arg = clap::Arg::new("config")
        .required(true)
        .value_name("FILE")
        .value_hint(clap::ValueHint::FilePath)
        .help("JSON file specifying related files for a heda project");

    let app_m = clap::command!()
        .arg(
            clap::Arg::new("debug")
                .short('d')
                .long("debug")
                .required(false)
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("Show debugging (verbose) output"),
        )
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("init")
                .about("Makes a default config pointing to skeleton files")
                .long_about(fmt_long_about(init_long_about(cso)))
                .arg(
                    clap::Arg::new("path")
                        .required(true)
                        .value_name("PATH")
                        .value_hint(clap::ValueHint::DirPath)
                        .help("Where the files will be initialized")
                        .long_help("Where the files will be initialized. It will fail if any "),
                ),
        )
        .subcommand(
            clap::Command::new("watch")
                .about("Validates JSON to a schema, then runs the derive, sort, and plot scripts")
                .arg(&config_arg),
        )
        .subcommand(
            clap::Command::new("migrate")
                .about("Backup then update a JSON file and its corresponding schema")
                .arg(&config_arg),
        )
        .subcommand(
            clap::Command::new("make-config-schema")
                .about("Generate JSON schema for heda config at the specified path")
                .arg(
                    clap::Arg::new("path")
                        .required(true)
                        .value_name("PATH")
                        .value_hint(clap::ValueHint::DirPath)
                        .help("Where the schema will be written"),
                ),
        )
        .get_matches();

    match app_m.subcommand() {
        Some(("init", sub_m)) => {
            let path_arg = sub_m.get_one::<String>("path").unwrap();
            let path = Path::new(path_arg);
            subcmd::init::run_init(path);
        }
        Some(("watch", sub_m)) => {
            let path_arg = sub_m.get_one::<String>("config").unwrap();
            subcmd::watch::run_watch(Path::new(path_arg));
        }
        Some(("migrate", sub_m)) => {
            let path_arg = sub_m.get_one::<String>("config").unwrap();
            subcmd::migrate::run_migrate(Path::new(path_arg));
        }
        Some(("make-config-schema", sub_m)) => {
            let path_arg = sub_m.get_one::<String>("path").unwrap();
            subcmd::make_config_schema::make_config_schema(Path::new(path_arg));
        }
        _ => {
            // clap should prevent this from being reached
            println!("Unknown subcommand. Try heda --help")
        }
    }
}

fn fmt_long_about(content: String) -> String {
    format!(
        "{}\n{}",
        console::style("About:").bold().underlined(),
        content
    )
}

struct CommonStyledObjects<'a> {
    pub file_config: StyledObject<&'a str>,
    pub file_data: StyledObject<&'a str>,
    pub file_schema: StyledObject<&'a str>,
    pub file_scripts: StyledObject<&'a str>,
    pub file_type: StyledObject<&'a str>,
    pub dir_plots: StyledObject<&'a str>,
    pub dir_backups: StyledObject<&'a str>,
    pub fn_derive: StyledObject<&'a str>,
    pub fn_sort: StyledObject<&'a str>,
    pub fn_plot: StyledObject<&'a str>,
    pub fn_migrate: StyledObject<&'a str>,
    pub cmd_watch: StyledObject<&'a str>,
    pub cmd_migrate: StyledObject<&'a str>,
    pub cmd_make_schema: StyledObject<&'a str>,
    pub spec_derive: StyledObject<&'a str>,
    pub spec_sort: StyledObject<&'a str>,
    pub spec_plot: StyledObject<&'a str>,
    pub spec_migrate: StyledObject<&'a str>,
}

impl CommonStyledObjects<'_> {
    fn new() -> Self {
        let cyan = console::Style::new().cyan();
        let blue = console::Style::new().blue();
        let magenta = console::Style::new().magenta();
        let bold = console::Style::new().bold();
        let italic = console::Style::new().italic();

        CommonStyledObjects {
            file_config: cyan.apply_to("config.json"),
            file_data: cyan.apply_to("data.json"),
            file_schema: cyan.apply_to("schema.json"),
            file_scripts: cyan.apply_to("scripts.rhai"), // only used once
            file_type: cyan.apply_to("type.rs"),
            dir_plots: blue.apply_to("plots"),
            dir_backups: blue.apply_to("backups"),
            fn_derive: magenta.apply_to("derive"),
            fn_sort: magenta.apply_to("sort"),
            fn_plot: magenta.apply_to("plot"),
            fn_migrate: magenta.apply_to("migrate"),
            cmd_watch: bold.apply_to("heda watch"),
            cmd_migrate: bold.apply_to("heda migrate"),
            cmd_make_schema: bold.apply_to("heda make-config-schema"),
            spec_derive: italic.apply_to("derive(Data) -> DerivedData"),
            spec_sort: italic.apply_to("sort(DerivedData) -> DerivedData"),
            spec_plot: italic.apply_to("plot(DerivedData) -> PlotDef"),
            spec_migrate: italic.apply_to("migrate(Datum) -> Datum"),
        }
    }
}

// TODO documentation in the program is good but accessing it here feels weird
// maybe just cut this, because it's awful anyway. include in commit 1 for fun
fn init_long_about(cso: CommonStyledObjects) -> String {
    fstrings::f!(
        "\
Generates default files in the provided directory.
Will panic instead of overwriting anything.
Creates the following:
  {cso.file_config}
\tSpecifies paths to the following files, relative to {cso.file_config}
\tIts schema can be generated with {cso.cmd_make_schema}
  {cso.file_data}
\tAn array of objects, to store human editable data defined by {cso.file_schema}
  {cso.file_schema}
\tJSON schema to validate {cso.file_data}, generated by {cso.file_type}
  {cso.file_scripts}
\tDefines the following functions; also uses {cso.file_type} to define Datum type
    {cso.fn_derive}
\t{cso.spec_derive}
\tUsed by {cso.cmd_watch} to manipulate the data for {cso.fn_derive} and {cso.fn_plot}
\tChanges are not written; use {cso.cmd_migrate} instead to update {cso.file_data}
    {cso.fn_sort}
\t{cso.spec_sort}
\tUsed by {cso.cmd_watch} to sort the data, using {cso.fn_derive} and before {cso.fn_plot}
    {cso.fn_plot}
\t{cso.spec_plot}
\tUsed by {cso.cmd_watch} to visualize the data after {cso.fn_derive} and {cso.fn_sort}
    {cso.fn_migrate}
\t{cso.spec_migrate}
\tUsed by {cso.cmd_migrate} to edit every entry in {cso.file_data}
  {cso.file_type}
\tDefines the type for each datum in the {cso.file_data} array
\tUsed to generate {cso.file_schema}
  {cso.dir_plots}
\tDirectory for the output of {cso.fn_plot}
  {cso.dir_backups}
\tDirectory for backups of data stored before {cso.fn_migrate}"
    )
}
