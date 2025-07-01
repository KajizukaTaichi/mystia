import { write, read } from "../ffi.mjs";
import fs from "fs";
import path from "path";

export class MystiaOSLib {
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
                    { type: "array", element: "int" },

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
            write_file: (path, content) => {
                fs.writeFileSync(
                    read(this.instance, "str", path),
                    read(this.instance, "str", content),
                    "utf8",
                );
            },
        };
    }
    set_wasm(instance) {
        this.instance = instance;
    }
    bridge() {
        const b = {};
        for (const k of Object.keys(this.functions)) {
            b[k] = (...a) => this.functions[k](...a);
        }
        return b;
    }
}
