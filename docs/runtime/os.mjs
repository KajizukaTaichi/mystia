import { write, read } from "./ffi.mjs";
import fs from "fs";
import path from "path";

export class OSLib {
    constructor() {
        this.functions = {
            getcwd: () => write(this.instance, "str", process.cwd()),
            remove: (value) => fs.unlinkSync(read(this.instance, "str", value)),
            mkdir: (value) => fs.mkdirSync(read(this.instance, "str", value)),
            rename: (src, dest) =>
                fs.renameSync(
                    read(this.instance, "str", src),
                    read(this.instance, "str", dest),
                ),
            chdir: (value) =>
                write(
                    this.instance,
                    "str",
                    process.chdir(read(this.instance, "str", value)),
                ),
            listdir: (value) =>
                write(
                    this.instance,
                    "str",
                    fs.readdirSync(read(this.instance, "str", value)),
                ),
            path_join: (value, end) =>
                write(
                    this.instance,
                    "str",
                    path.join(
                        read(this.instance, "str", value),
                        read(this.instance, "str", end),
                    ),
                ),
            path_basename: (value) =>
                write(
                    this.instance,
                    "str",
                    path.basename(read(this.instance, "str", value)),
                ),
            path_parent: (value) =>
                write(
                    this.instance,
                    "str",
                    path.dirname(read(this.instance, "str", value)),
                ),
            path_abs: (value) =>
                write(
                    this.instance,
                    "str",
                    path.resolve(read(this.instance, "str", value)),
                ),
            path_exist: (value) =>
                fs.existsSync(read(this.instance, "str", value)),
            path_isfile: (value) => {
                let isFile = false;
                try {
                    isFile = fs
                        .statSync(read(this.instance, "str", value))
                        .isFile();
                } catch {
                    isFile = false;
                }
                return isFile;
            },
            path_isdir: (value) => {
                let isDir = false;
                try {
                    isDir = fs
                        .statSync(read(this.instance, "str", value))
                        .isDirectory();
                } catch {
                    isDir = false;
                }
                return isDir;
            },
            path_isabs: (value) =>
                path.isAbsolute(read(this.instance, "str", value)),
            path_root: (value) =>
                write(
                    this.instance,
                    "str",
                    path.parse(read(this.instance, "str", value)).root,
                ),
            path_ext: (value) =>
                write(
                    this.instance,
                    "str",
                    path.extname(read(this.instance, "str", value)),
                ),
            read_file: (value) =>
                write(
                    this.instance,
                    "str",
                    fs.readFileSync(read(this.instance, "str", value), "utf8"),
                ),
            write_file: (value) => {
                fs.writeFileSync(
                    read(this.instance, "str", value),
                    read(this.instance, "str", value),
                    "utf8",
                );
            },
        };
    }
    set_wasm(instance) {
        this.instance = instance;
    }
    bridge() {
        return {
            getcwd: () => this.functions.getcwd(),
            remove: (value) => this.functions.remove(value),
            mkdir: (value) => this.functions.mkdir(value),
            rename: (from, to) => this.functions.rename(from, to),
            chdir: (value) => this.functions.chdir(value),
            listdir: (value) => this.functions.listdir(value),
            path_join: (value) => this.functions.path_join(value),
            path_basename: (value) => this.functions.path_basename(value),
            path_parent: (value) => this.functions.path_parent(value),
            path_abs: (value) => this.functions.path_abs(value),
            path_exist: (value) => this.functions.path_exist(value),
            path_isfile: (value) => this.functions.path_isfile(value),
            path_isdir: (value) => this.functions.path_isdir(value),
            path_isabs: (value) => this.functions.path_isabs(value),
            path_root: (value) => this.functions.path_root(value),
            path_ext: (value) => this.functions.path_ext(value),
            read_file: (value) => this.functions.read_file(value),
            write_file: (a, b) => this.functions.write_file(a, b),
        };
    }
}
