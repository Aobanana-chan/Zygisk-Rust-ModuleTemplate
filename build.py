import os
import subprocess
import zipfile
import shutil

target_abi = ["armeabi-v7a", "arm64-v8a", "x86", "x86_64"]
out_dir = "./out"
profile = "release"

module_config = {
    "id": "rust_zygisk",
    "name": "RustZygisk",
    "version": "v1.0.0",
    "versionCode": 1,
    "author": "Aobanana",
    "description": "A Rust Zygisk Module Template"
}

def build():
    targets = " ".join([f"-t {abi}" for abi in target_abi])
    subprocess.run(f"cargo ndk {targets} -o {out_dir} build --{profile}")


def pack_module():
    if not os.path.exists(os.path.join(out_dir, "zygisk")):
        os.mkdir(os.path.join(out_dir, "zygisk"))
    for foldername, subfolders, filenames in os.walk(out_dir):
        if not foldername.endswith(tuple(target_abi)):
            continue
        for filename in filenames:
            if filename.endswith(".so"):
                # new_filename = os.path.join(foldername, f"{os.path.basename(foldername)}.so")
                src = os.path.join(foldername, filename)
                dst = os.path.join(os.path.dirname(foldername), "zygisk",f"{os.path.basename(foldername)}.so")
                if os.path.exists(dst):
                    os.remove(dst)
                os.rename(src, dst)
            else:
                raise Exception("expect only one file $libname.so in $abi dir, cargo ndk may too new and build scprit need to maintain")

    # clean
    for abi_folder in target_abi:
        shutil.rmtree(os.path.join(out_dir, abi_folder))
    
    zip_name = f"{module_config['id']}-{module_config['version']}-{profile}.zip"
    with zipfile.ZipFile(os.path.join(out_dir, zip_name), 'w', zipfile.ZIP_DEFLATED) as zipf:
        for root, _, files in os.walk(out_dir):
            for file in files:
                if file != zip_name:  # exclude zip
                    file_path = os.path.join(root, file)
                    zipf.write(file_path, os.path.relpath(file_path, out_dir))

def write_module_info():
    with open(os.path.join(out_dir, "module.prop"), "r+t") as file:
        file_content = file.read()
        for key, value in module_config.items():
            file_content = file_content.replace(f"${{{key}}}", str(value))
        file.seek(0)
        file.truncate()
        file.write(file_content)

def copy_folder_contents(source_folder, destination_folder):
    for root, dirs, files in os.walk(source_folder):
        dest_root = root.replace(source_folder, destination_folder, 1)
        if not os.path.exists(dest_root):
            os.makedirs(dest_root)

        for file in files:
            source_file = os.path.join(root, file)
            dest_file = os.path.join(dest_root, file)
            shutil.copy2(source_file, dest_file)

if __name__ == "__main__":
    build()
    copy_folder_contents("./template", out_dir)
    write_module_info()
    pack_module()