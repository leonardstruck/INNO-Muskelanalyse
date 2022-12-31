const fs = require('fs')
const { default: dynamic } = require('next/dynamic')
const path = require('path')

let extension = ''
if (process.platform === 'win32') {
    extension = '.exe'
}

async function main() {
    const { execa } = await import('execa');

    const rustInfo = (await execa('rustc', ['-vV'])).stdout
    const targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
    if (!targetTriple) {
        console.error('Failed to determine platform target triple')
    }

    const paths = [
        "resources/dist/test"
    ];

    paths.forEach(path => {
        fs.renameSync(
            `${path}${extension}`,
            `${path}-${targetTriple}${extension}`
        )
    });
}

main().catch((e) => {
    throw e
})