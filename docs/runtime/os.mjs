import { write, read } from "./ffi.mjs";
import fs from "fs";
import path from "path";

export class OSLib {
    constructor() {
        this.functions = {
            getcwd: () => {
                return write(this.instance, "str", process.cwd());
            },
            // env: () => Math.PI,
            remove: (value) => {
                fs.unlinkSync(read(this.instance, "str", value));
            },
            mkdir: (value) => {
                fs.mkdirSync(read(this.instance, "str", value));
            },
            rename: (src, dest) => {
                return fs.renameSync(
                    read(this.instance, "str", src),
                    read(this.instance, "str", dest),
                );
            },
            chdir: (value) => {
                return write(
                    this.instance,
                    "str",
                    process.chdir(read(this.instance, "str", value)),
                );
            },
            listdir: (value) => {
                return write(
                    this.instance,
                    "str",
                    fs.readdirSync(read(this.instance, "str", value)),
                );
            },
            path_join: (value, end) => {
                return write(
                    this.instance,
                    "str",
                    path.join(
                        read(this.instance, "str", value),
                        read(this.instance, "str", end),
                    ),
                );
            },
            path_basename: (value) => {
                return write(
                    this.instance,
                    "str",
                    path.basename(read(this.instance, "str", value)),
                );
            },
            path_parent: (value) => {
                return write(
                    this.instance,
                    "str",
                    path.dirname(read(this.instance, "str", value)),
                );
            },
            path_abs: (value) => {
                return write(
                    this.instance,
                    "str",
                    path.resolve(read(this.instance, "str", value)),
                );
            },
            path_exist: (value) => {
                return fs.existsSync(read(this.instance, "str", value));
            },
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
            path_isabs: (value) => {
                return path.isAbsolute(read(this.instance, "str", value));
            },
            //path_split: (value) => Math.exp(value),
            path_root: (value) => {
                return write(
                    this.instance,
                    "str",
                    path.parse(read(this.instance, "str", value)).root,
                );
            },
            // path_match: (p,s) => {
            //     const name = path.basename(p);
            //     const escaped = s.replace(/[-\/\\^$+?.()|[\]{}]/g, "\\$&");
            //     const regexStr = '^' + escaped.replace(/\\\*/g, '.*').replace(/\\\?/g, '.') + '$';
            //     return new RegExp(regexStr).test(name);
            // },
            // path_glob: (p,s) => {
            //     if (!fs.existsSync(p) || !fs.statSync(p).isDirectory()) return [];
            //         const entries = fs.readdirSync(p);
            //         const escaped = s.replace(/[-\/\\^$+?.()|[\]{}]/g, "\\$&");
            //         const regexStr = '^' + escaped.replace(/\\\*/g, '.*').replace(/\\\?/g, '.') + '$';
            //         return entries
            //           .filter(e => new RegExp(regexStr).test(e))
            //           .map(e => new Path(path.join(p, e)));
            // },
            // path_rglob: (p,s) => {
            //     const results = [];
            //     const escaped = s.replace(/[-\/\\^$+?.()|[\]{}]/g, "\\$&");
            //     const regexStr = '^' + escaped.replace(/\\\*/g, '.*').replace(/\\\?/g, '.') + '$';
            //     const regex = new RegExp(regexStr);
            //     const recurse = dir => {
            //         const entries = fs.readdirSync(dir);
            //         for (const entry of entries) {
            //         const full = path.join(dir, entry);
            //         const stat = fs.statSync(full);
            //         if (regex.test(entry)) {
            //             results.push(new Path(full));
            //         }
            //         if (stat.isDirectory()) {
            //             recurse(full);
            //         }
            //         }
            //     };
            //     if (fs.existsSync(p) && fs.statSync(p).isDirectory()) {
            //         recurse(p);
            //     }
            //     return results;
            // },
            path_ext: (value) => {
                return write(
                    this.instance,
                    "str",
                    path.extname(read(this.instance, "str", value)),
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
            //path_match: (path, pattern) => this.functions.path_match(path,pattern),
            //path_glob: (path, pattern) => this.functions.path_glob(path,pattern),
            //path_rglob: (path, pattern) => this.functions.path_rglob(path,pattern),
            path_ext: (value) => this.functions.path_ext(value),
        };
    }
}
