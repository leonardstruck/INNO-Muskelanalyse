import { invoke } from "@tauri-apps/api/tauri";
import { Micrograph } from "../../../src-tauri/bindings/Micrograph";

type HandleImportProps = {
    linkToCase?: number;
};

const handleImport = async ({ linkToCase }: HandleImportProps) => {
    const open = (await import("@tauri-apps/api/dialog")).open;

    return new Promise<void>(async (resolve, reject) => {
        const files = await open({
            filters: [
                {
                    name: "Images",
                    extensions: ["jpg", "png", "gif"]
                }
            ],
            multiple: true
        });

        // resolve early if no files are selected
        if (files == null) {
            resolve()
        }

        invoke("import_micrographs", {
            micrographPaths: files, linkToCase
        }).then(() => {
            resolve()
        }).catch((reason) => {
            reject(reason);
        })
    })

}

type FetchMicrographsProps = {
    caseId?: number;
};

const fetchMicrographs = ({ caseId }: FetchMicrographsProps): Promise<Micrograph[]> => {
    return invoke("get_micrographs", { caseId }).then((response: string) => JSON.parse(response) as Micrograph[]);
}

export { handleImport, fetchMicrographs };