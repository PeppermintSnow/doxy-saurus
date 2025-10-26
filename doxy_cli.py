import argparse
import os
from .doxy_extractor import extractor
from .doxy_parser import parser

def cli():
    description = "Convert doxygen comment blocks to Docusaurus-tailored Markdown."
    arg_parser = argparse.ArgumentParser(description=description)
    arg_parser.add_argument('input_dir', help="Directory containing files to read from")
    arg_parser.add_argument('output_dir', help="Directory where files should be saved")
    arg_parser.add_argument('--subdirs', nargs='*', help="Subdirectories to read (default: include)", default=["include"])
    arg_parser.add_argument('-e', '--extensions', nargs='*', help="File extensions to read", default=["h"])
    args = arg_parser.parse_args()
    input_dir = os.path.abspath(args.input_dir)
    output_dir = os.path.abspath(args.output_dir)
    subdirs = [subdir.replace("./", "") for subdir in args.subdirs]
    extensions = [ext.replace(".", "") for ext in args.extensions]

    docs = extractor(input_dir, subdirs, extensions)
    markdown_list = parser(docs)

if __name__ == '__main__':
    cli()
