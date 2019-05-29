use super::prelude::*;

group!({
    name: "experimental",
    options: {
        owners_only: true,
    },
    commands: [exp1]
});

#[command]
fn exp1(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx, |m| {
        m.content("Hello, World!").embed(|e| {
            e.title("This is a title")
                .description("This is a description")
                .fields(vec![
                    ("This is the first field", "This is a field body", true),
                    (
                        "This is the second field",
                        "Both of these fields are inline",
                        true,
                    ),
                ])
                .field(
                    "This is the third field",
                    "This is not an inline field",
                    false,
                )
                .footer(|f| f.text("This is a footer"))
                .colour((246, 111, 0))
        })
    })?;
    Ok(())
}
