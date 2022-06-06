#include "argparse.hpp"
#include "emulator.hpp"


int main(int argc, char *argv[]) {
    argparse::ArgumentParser parser("cpp_bf_emulator");

    parser.add_argument("-i", "--input")
           .default_value(std::string("-"))
           .required()
           .help("specify input file or use stdin");
    parser.add_argument("-m", "--memory")
           .default_value(consts::DEFAULT_MEM_SIZE)
           .required()
           .scan<'i', std::size_t>()
           .help("specify memory size");

    try {
        parser.parse_args(argc, argv);
    } catch (std::runtime_error &err) {
        std::cerr << err.what() << std::endl;
        std::cerr << parser;
        std::exit(1);
    }

    std::size_t mem_size = parser.get<std::size_t>("--memory");
    std::string filename = parser.get<std::string>("--input");

    Emulator app = Emulator(mem_size);
    if (filename == "-") {
        std::string buffer;
        std::cout << "> ";
        std::cin >> buffer;
        app.from_buffer(buffer);
    } else {
        app.from_file(filename.c_str());
    }
    app.execute();

    return EXIT_SUCCESS;
}
