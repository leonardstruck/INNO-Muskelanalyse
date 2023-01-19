import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"
import { Micrograph } from "../../../src-tauri/bindings/Micrograph"
import Loading from "../layout/Loading"
import Empty from "./Empty"
import ImageCard from "./ImageCard"

export type ImageBrowserProps = {
    caseId?: number
}

const ImageBrowser = ({ caseId }: ImageBrowserProps) => {
    const [isLoading, setIsLoading] = useState(true);
    const [isError, setIsError] = useState(false);
    const [images, setImages] = useState<Micrograph[]>([]);

    useEffect(() => {
        if (!caseId) {
            // TODO: Fetch all images
        } else {
            invoke("get_micrographs_by_case", { queryCaseId: caseId }).then((res: string) => {
                setImages(JSON.parse(res) as Micrograph[]);
                setIsLoading(false);
            }).catch((err) => {
                setIsError(true);
                console.error(err);
            })
        }
    }, [caseId])

    if (isError) return <div>Beim Abrufen der Aufnahmen ist ein Fehler aufgetreten</div>
    if (isLoading) return <Loading />
    if (images.length === 0) return <Empty {...{ setIsLoading, caseId }} />

    return (
        <ul role="list" className="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8">
            {images.map((image) => <ImageCard {...{ image }} />)}
        </ul>
    )
}

const fetchImages = async () => {

}

export default ImageBrowser