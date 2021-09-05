# -----------------------------
# Options effecting formatting.
# -----------------------------
with section("format"):  # noqa: undefined name 'section'flake8(F821)
    line_width = 120
    tab_size = 4

    # If true, separate flow control names from their parentheses with a space
    separate_ctrl_name_with_space = True

    # If true, separate function names from parentheses with a space
    separate_fn_name_with_space = True

    # If a statement is wrapped to more than one line, than dangle the closing parenthesis on its own line.
    dangle_parens = True
