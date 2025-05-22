import { write, read } from "./ffi.mjs";

export class OSLib {
    constructor() {
        this.functions = {
            getcwd: () => {return write(this.instance, "str", process.cwd());},
            // env: () => Math.PI,
            remove: (value) => fs.unlinkSync(value),
            mkdir: (value) => fs.mkdirSync(value),
            rename: (src,dest) => fs.renameSync(src, dest),
            chdir: (value) => process.chdir(value),
            listdir: (value) => fs.readdirSync(value),
            path_join: (value, end) => path.join(value, end),
            path_basename: (value) => path.basename(value),
            path_dirname: (value) => path.dirname(value),
            path_abs: (value) => path.resolve(value),
            path_exist: (value) => fs.existsSync(value),
            path_isfile: (value) => {
                let isFile = false;
                try {
                isFile = fs.statSync(value).isFile();
                } catch {
                isFile = false;
                }
                return isFile;
            },
            path_isdir: (value) => {
                let isDir = false;
                try {
                isDir = fs.statSync(value).isDirectory();
                } catch {
                isDir = false;
                }
                return isDir;
            },
            path_isabs: (value) => path.isAbsolute(value),
            //path_split: (value) => Math.exp(value),
            path_root: (value) => path.parse(value).root,
            path_ext: (value) => path.extname(value),
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
            path_dirname: (value) => this.functions.path_dirname(value),
            path_abs: (value) => this.functions.path_abs(value),
            path_exist: (value) => this.functions.path_exist(value),
            path_isfile: (value) => this.functions.path_isfile(value),
            path_isdir: (value) => this.functions.path_isdir(value),
            path_isabs: (value) => this.functions.path_isabs(value),
            path_root: (value) => this.functions.path_root(value),
            path_ext: (value) => this.functions.path_ext(value),
        };
    }
}