import { invoke } from "@tauri-apps/api/tauri";
import { Micrograph } from "../../../src-tauri/bindings/Micrograph";

type HandleImportProps = {
    caseId?: number;
};

const handleImport = async (url, { arg }: { arg: HandleImportProps }) => {
    const { caseId } = arg;

    const open = (await import("@tauri-apps/api/dialog")).open;

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
        return;
    }

    return invoke("import_micrographs", {
        micrographPaths: files, caseId
    })
}

type FetchMicrographsProps = {
    caseId?: number;
};

const fetchMicrographs = ({ caseId }: FetchMicrographsProps): Promise<Micrograph[]> => {
    return invoke("get_micrographs", { caseId }).then((response: string) => JSON.parse(response) as Micrograph[]);
}


export { handleImport, fetchMicrographs };