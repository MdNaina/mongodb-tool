const fs = require('fs');
const os = require('os')
const axios = require('axios');
const path = require("path");
const decompress = require('decompress');
const decompressTargz = require('decompress-targz');

const VERSION = "100.7.2"

const platformUrlObj = {
  "windows-latest": "windows-x86_64",
  "ubuntu-20.04": "ubuntu2004-x86_64",
  "macos-latest": "macos-x86_64"
}
console.log(process.argv)
const platformName = process.argv.at(2)

console.log(platformUrlObj[platformName], platformName)
if (!platformUrlObj[platformName]) {
  console.error(`platform incorrect please select from this (${Object.keys(platformUrlObj).join(",")})`)
  process.exit(1)
}
// Specify the URL of the compressed file
const url = `https://fastdl.mongodb.org/tools/db/mongodb-database-tools-${platformUrlObj[platformName]}-${VERSION}.${platformName === "ubuntu-20.04" ? "tgz" : "zip"}`;

// Specify the target path to save the compressed file
const tempDir = path.join(os.tmpdir(), "mongodb-tools")
const distDir = path.join(process.cwd(), "src-tauri", "binaries")

if (!fs.existsSync(tempDir))
  fs.mkdirSync(tempDir, { recursive: true })

if (!fs.existsSync(distDir))
  fs.mkdirSync(distDir, { recursive: true })

const targetPath = path.join(tempDir, "archive");

const suffixObj = {
  "windows-latest": "x86_64-pc-windows-msvc",
  "ubuntu-20.04": "x86_64-unknown-linux-gnu",
  "macos-latest": "x86_64-apple-darwin"
}

const requireProgram = {
  mongodump: "db-dump",
  mongorestore: "db-restore",
}

const downloadAndExtractFiles = async (url, targetPath) => {
  try {
    // Download the compressed file
    const response = await axios.get(url, { responseType: 'arraybuffer' });
    const compressedFile = Buffer.from(response.data);

    // Save the compressed file
    fs.writeFileSync(targetPath, compressedFile);

    const extractionTempPath = `${targetPath.replace('.tgz', '')}-extraction-temp`;
    if (!fs.existsSync(extractionTempPath))
      fs.mkdirSync(extractionTempPath, { recursive: true })
    // Extract specific files from the compressed file
    await decompress(targetPath, extractionTempPath, {
      // plugins: url.split(".").at(-1) == "tgz" ? [decompressTargz()] : [],
    });

    // Move extracted files to the desired location and delete the extraction-temp folder
    fs.readdirSync(extractionTempPath, { recursive: true }).forEach(file => {
      const sourcePath = path.join(extractionTempPath, file);
      const { name, ext } = path.parse(file)
      console.log({ name })
      if (requireProgram[name]) {
        path.join()
        const destinationPath = path.join(distDir, `${requireProgram[name]}-${suffixObj[platformName]}${ext}`);
        fs.renameSync(sourcePath, destinationPath);
      }
    });
    fs.rmSync(extractionTempPath, { recursive: true });
    console.log('Extraction complete!');
  } catch (error) {
    console.error('Error:', error.message);
  }
};


// Call the function to download and extract the files
downloadAndExtractFiles(url, targetPath).then(() => {
  fs.readdir(distDir, (err, files) => {
    if (err) {
      console.error('Error reading directory:', err);
    } else {
      console.log('Files in the directory:');
      files.forEach((file) => {
        console.log(file);
      });
    }
  });
});
