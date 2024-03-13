import gleam/string

pub fn message(log_line: String) -> String {
  let log_line = string.trim(log_line)
  let assert Ok(#(level, msg)) = string.split_once(log_line, ":")
  msg
  |> string.trim()
}

pub fn log_level(log_line: String) -> String {
  let assert Ok(#(level, msg)) = string.split_once(log_line, ":")
  level
  |> string.drop_left(1)
  |> string.drop_right(1)
  |> string.lowercase()
}

pub fn reformat(log_line: String) -> String {
  message(log_line) <> " (" <> log_level(log_line) <> ")"
}
