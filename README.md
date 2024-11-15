# MHWs-Ext-Manager

A powerful tool for managing `.pak` files for **Monster Hunter Wilds**. This tool allows you to merge, unmerge, and split `.list` files efficiently.

---

## Features

- **Merge**: Combines extracted folders (`re_chunk_000` and `re_chunk_000.pak.sub_000`) into a single folder.
- **Unmerge**: Splits a merged folder back into `re_chunk_000` and `re_chunk_000.pak.sub_000` using `.list` files.
- **Split**: Splits a `.list` file into separate files for `re_chunk_000` and `re_chunk_000.pak.sub_000`.
- **Automatic File Download**:
  - If `RETool.exe` or `MHWs_STM_Beta.list` is missing, the script will automatically download them.

---

## Requirements

- **Operating System**: Windows 10 x64 or later
- **Dependencies**:
  - PowerShell (for the batch script)
  - Internet connection (for downloading missing files)

---

## Usage

### 1. **Extract `.pak` Files**
1. Drag and drop a `.pak` file onto `extract-wilds-pak.bat`.
2. The script will:
   - Check for the existence of `RETool.exe` and `MHWs_STM_Beta.list`.
   - Automatically download missing files from their respective GitHub sources.
   - Extract the `.pak` file using `RETool.exe`.

---

### 2. **Merge Folders**
Run the following command to merge folders:

```bash
MHWs-Ext-Manager.exe merge <source_folder> <destination_folder>
```

**Example**:
```bash
MHWs-Ext-Manager.exe merge re_chunk_000.pak.sub_000 re_chunk_000
```

---

### 3. **Unmerge Folders**
Run the following command to split a merged folder back into two separate folders:

```bash
MHWs-Ext-Manager.exe unmerge <merged_folder> <list_file_for_re_chunk> <list_file_for_sub_chunk>
```

**Example**:
```bash
MHWs-Ext-Manager.exe unmerge re_chunk_000(merged) re_chunk_000.list re_chunk_000.pak.sub_000.list
```

---

### 4. **Split `.list` File**
Run the following command to split a `.list` file:

```bash
MHWs-Ext-Manager.exe split <input_list_file>
```

**Example**:
```bash
MHWs-Ext-Manager.exe split MHWs_STM_Beta.list
```

**Important**:
- Before using the `split` function, ensure that `re_chunk_000.pak` and `re_chunk_000.pak.sub_000.pak` have been extracted into their respective directories (`re_chunk_000` and `re_chunk_000.pak.sub_000`).
- The `split` function will compare the list entries with the extracted directories to generate accurate output files:
  - `re_chunk_000.list`
  - `re_chunk_000.pak.sub_000.list`

---

## Notes
- If `RETool.exe` or `MHWs_STM_Beta.list` is missing, the batch script will automatically download them:
  - `RETool.exe` will be downloaded from [RETool GitHub Repository](https://raw.githubusercontent.com/mhvuze/MonsterHunterRiseModding/main/files/REtool.exe).
  - `MHWs_STM_Beta.list` will be downloaded from [Ekey's GitHub Repository](https://github.com/Ekey/REE.PAK.Tool).
- Ensure that `RETool.exe` and `MHWs_STM_Beta.list` are located in the same directory as the batch script after the first run.
- The tool is optimized for Monster Hunter Wilds `.pak` files but may work with similar formats.

---

## License
This project is licensed under the MIT License.
