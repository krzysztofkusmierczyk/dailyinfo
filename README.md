# DailyInfo

Sends a Slack message about Polish name days, festivites, and more.

## Running

1. Create a [Slack app with webhook support](https://api.slack.com/messaging/webhooks).
1. Download the latest release.
1. Create an `.env` file with `SLACK_WEBHOOK=<you slack webhook>` in the same directory as the executable.
1. Make it executable (chmod +x dailyinfo).
1. Run it `./dailyinfo`.

## Development

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Clone the repository.
1. Run `cp .env.example .env`.
1. Set all variables in `.env` file.
1. Run `cargo run`.

Check [Slack block kit documentation](https://api.slack.com/reference/block-kit/blocks) for all message structures. Use [block kit builder](https://app.slack.com/block-kit-builder) for prototyping Slack messages.

## Contributions

Bug fixes only.
