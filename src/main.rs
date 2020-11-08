use anyhow::anyhow;
use clap::{clap_app, ArgMatches};

fn main() -> anyhow::Result<()> {
    let matches = clap_app!(("bloaty-pgp") =>
        (version: "1.0")
        (author: "Alex Komissarov <k.a.komissar@gmail.com>")
        (about: "PGP tool written in Rust.")
        (@subcommand verify =>
            (about: "verify a clearsigned message")
            (@arg source: -s --source +takes_value
                "Sets the source file containing the mesage to verify. Defaults to 'msg.txt.asc'.")
            (@arg publickey: --publickey + takes_value
             "Sets the public key containing the public ey which verifies the \
             message. Defaults to 'publice.pgp'.")
        )
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("verify") {
        verify(matches)
    } else {
        Err(anyhow!("unknown subcommand"))
    }
}

fn verify(matches: &ArgMatches) -> anyhow::Result<()> {
    let source = matches.value_of("source").unwrap_or("msg.tt.asc");
    let public_key_path = matches.value_of("publickey").unwrap_or("public.pgp");

    bloaty_pgp::verify_cleartext_message(source, public_key_path)
}
