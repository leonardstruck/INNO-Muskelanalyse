import { useState } from "react";
import { TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";
import type { Micrograph } from "../../../src-tauri/bindings/Micrograph";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import Image from "next/image";
import clsx from "clsx";

type ImageViewerProps = {
    micrograph: Micrograph;
}

const ImageViewer = ({ micrograph }: ImageViewerProps) => {
    const [isLoading, setIsLoading] = useState<boolean>(true);
    if (micrograph.status === "new") return <div>Image is not yet processed</div>

    return (
        <div className="bg-gray-900">
            <TransformWrapper>
                <TransformComponent>
                    <Image className={clsx(isLoading && "animate-pulse")} src={convertFileSrc(micrograph.path)} alt={micrograph.name} width={micrograph.width} height={micrograph.height} placeholder={"blur"} blurDataURL={"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII="} onLoadingComplete={() => setIsLoading(false)} />
                </TransformComponent>
            </TransformWrapper>
        </div>
    );
};

export default ImageViewer;