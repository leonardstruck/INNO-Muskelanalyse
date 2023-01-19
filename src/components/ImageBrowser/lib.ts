import React from "react";
import { flushSync } from "react-dom";
import { invoke } from "@tauri-apps/api/tauri";

type HandleImportProps = {
    setLoadingState: React.Dispatch<React.SetStateAction<boolean>>;
    linkToCase?: number;
};

const handleImport = async ({ setLoadingState, linkToCase }: HandleImportProps) => {
    const open = (await import("@tauri-apps/api/dialog")).open;

    // wrap in flushSync to ensure the loading state is set before the dialog is opened
    flushSync(() => setLoadingState(true));

    await open({
        filters: [
            {
                name: "Images",
                extensions: ["jpg", "png", "gif"],
            },
        ],
        multiple: true,
    }).then((res) => {
        if (res == null) {
            setLoadingState(false);
            return;
        }

        invoke("import_micrographs", { micrographPaths: res, linkToCase }).then(() => {
            setLoadingState(false);
        });

    });
}

export { handleImport };