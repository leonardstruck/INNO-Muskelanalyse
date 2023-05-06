import { invoke } from "@tauri-apps/api/tauri";
import { PortableMicrograph } from "../../../src-tauri/bindings/PortableMicrograph"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import EmptyState from "../../components/micrographs/EmptyState";
import { Tab } from "@headlessui/react";
import List from "../../components/micrographs/List";
import clsx from "clsx";
import { LayoutGrid, LayoutList } from "lucide-react";
import { Button } from "../../components/ui/button";

const MicrographsPage = () => {
    const { data, refetch } = useQuery(["micrographs"], micrographFetcher, {
        refetchInterval: 5000
    });
    const queryClient = useQueryClient();
    const { mutate: mutate_import } = useMutation(["import_micrographs"], importMicrographs, {
        onSuccess: () => {
            refetch();
            queryClient.invalidateQueries(["queue_status"]);
        }
    });
    const { mutate: mutate_delete } = useMutation(["delete_micrograph"], deleteMicrograph, {
        onSuccess: () => {
            refetch();
        }
    });

    if (!data) {
        return (
            <div>no data</div>
        )
    }

    if (data.length === 0) {
        return (
            <div className="flex justify-center items-center h-full">
                <EmptyState onImport={mutate_import} />
            </div>
        )
    }

    return (
        <Tab.Group>
            <Tab.List className="flex justify-between">
                <div>
                    <Button variant={"secondary"} onClick={() => mutate_import()}>Import Micrograph</Button>
                </div>
                <div>
                    <Tab>{({ selected }) => (<TabItem {...{ selected }}><LayoutList className="h-4" /></TabItem>)}</Tab>
                    <Tab>{({ selected }) => (<TabItem {...{ selected }}><LayoutGrid className="h-4" /></TabItem>)}</Tab>
                </div>
            </Tab.List>
            <Tab.Panels className={"mt-4"}>
                <Tab.Panel><List micrographs={data} onDelete={mutate_delete} /></Tab.Panel>
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

const deleteMicrograph = async (uuid: string) => {
    const dialog = await import("@tauri-apps/api/dialog");

    const result = await dialog.confirm("Are you sure you want to delete this micrograph?", {
        title: "Delete Micrograph",
        type: "warning",
        cancelLabel: "Cancel",
        okLabel: "Delete"
    });

    if (!result) {
        return
    }

    return invoke("delete_micrograph", { id: uuid })
}

export default MicrographsPage;