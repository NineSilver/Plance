// Main project files
pub const C_MAIN: &str = "#include <stdio.h>

int main(int argc, char* argv[])
{
    printf(\"Hello, World\\n\");
    return 0;
}
";

pub const CPP_MAIN: &str = "#include <iostream>

using namespace std;

int main(int argc, char* argv[])
{
    cout << \"Hello, World!\" << endl;
    return 0;
}
";

pub const DEFAULT_MAIN: &str = "Automatically generated with 💖 by Plance\n";

pub const C_FILE: &str = "main.c";
pub const CPP_FILE: &str = "main.cpp";
pub const DEFAULT_FILE: &str = "default.file";
