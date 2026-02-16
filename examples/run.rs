use dfhack_remote::CoreRunCommandRequest;

fn main() {
    env_logger::init();
    let mut client = dfhack_remote::connect().unwrap();
    let command = CoreRunCommandRequest {
        command: "lua".to_string(),
        arguments: vec![
            "print(dfhack.df2utf(dfhack.translation.translateName(df.global.world.world_data.active_site[0].name)))".to_string()
        ]
    };
    let reply = client.core().run_command(command).unwrap();

    let command = CoreRunCommandRequest {
        command: "lua".to_string(),
        arguments: vec![
            "print(dfhack.df2utf(dfhack.translation.translateName(df.global.world.world_data.active_site[0].name, true)))".to_string()
        ]
    };
    let reply_en = client.core().run_command(command).unwrap();

    println!(
        "The fortress is named {} ({})",
        reply.fragments[0].text.trim(),
        reply_en.fragments[0].text.trim()
    );
}
