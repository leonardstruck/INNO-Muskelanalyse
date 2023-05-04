import { invoke } from "@tauri-apps/api/tauri";
import { PortableMicrograph } from "../../../src-tauri/bindings/PortableMicrograph"
import { useMutation, useQuery } from "@tanstack/react-query";
import EmptyState from "../../components/micrographs/EmptyState";

const MicrographsPage = () => {
    const { data } = useQuery(["micrographs"], micrographFetcher);
    const { mutate } = useMutation(["import_micrographs"], importMicrographs);

    if (!data) {
        return (
            <div>no data</div>
        )
    }

    if (data.length === 0) {
        return (
            <div className="flex justify-center items-center h-full">
                <EmptyState onImport={mutate} />
            </div>
        )
    }

    return (
        <div>
            {data.map((micrograph) => {
                return (
                    <div key={micrograph.uuid}>
                        {micrograph.uuid}
                    </div>
                )
            })}
        </div>
    )
}

const micrographFetcher = async () => {
    return invoke("get_micrographs").then((res) => {
        return res as PortableMicrograph[]
    })
}

const importMicrographs = async () => {
    const dialog = await import("@tauri-apps/api/dialog");

    const result = await dialog.open({
        filters: [{
            name: "Images",
            extensions: ["png", "jpg", "jpeg", "tiff", "tif", "bmp"]
        }],
        multiple: true
    })

    if (!result) {
        return
    }

    return invoke("import_micrographs", { files: result })
}

export default MicrographsPage;