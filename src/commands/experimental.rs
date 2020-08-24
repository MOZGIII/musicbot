use super::prelude::*;

#[group]
#[owners_only]
#[commands(exp1)]
struct Experimental;

#[command]
pub async fn exp1(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |m| {
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
        })
        .await?;
    Ok(())
}
