import { invoke } from "@tauri-apps/api/tauri";
import { PortableMicrograph } from "../../../src-tauri/bindings/PortableMicrograph"
import { useMutation, useQuery } from "@tanstack/react-query";
import EmptyState from "../../components/micrographs/EmptyState";
import { Tab } from "@headlessui/react";
import List from "../../components/micrographs/List";
import clsx from "clsx";
import { LayoutGrid, LayoutList } from "lucide-react";
import { Button } from "../../components/ui/button";

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
        <Tab.Group>
            <Tab.List className="flex justify-between">
                <div>
                    <Button variant={"secondary"} onClick={() => mutate()}>Import Micrograph</Button>
                </div>
                <div>
                    <Tab>{({ selected }) => (<TabItem {...{ selected }}><LayoutList className="h-4" /></TabItem>)}</Tab>
                    <Tab>{({ selected }) => (<TabItem {...{ selected }}><LayoutGrid className="h-4" /></TabItem>)}</Tab>
                </div>
            </Tab.List>
            <Tab.Panels className={"mt-4"}>
                <Tab.Panel><List micrographs={data} /></Tab.Panel>
                <Tab.Panel>Grid View</Tab.Panel>
            </Tab.Panels>
        </Tab.Group>
    )
}

type TabProps = {
    children: React.ReactNode,
    selected: boolean
}

const TabItem = ({ children, selected }: TabProps) => {
    return (
        <div className={clsx(selected && "bg-gray-100", "text-gray-600 rounded-md px-2 py-2 text-sm font-medium")}>
            {children}
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