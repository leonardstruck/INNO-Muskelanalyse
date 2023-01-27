import { useEffect, useRef, useState } from "react";
import { TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";
import type { Micrograph } from "../../../src-tauri/bindings/Micrograph";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import Image from "next/image";
import clsx from "clsx";
import { Segment } from "../../../src-tauri/bindings/Segment";
import { Toggle, ToggleItem } from "@tremor/react";
import { useRouter } from "next/router";

import { EyeIcon, EyeSlashIcon } from "@heroicons/react/24/outline";

type ImageViewerProps = {
    micrograph: Micrograph
    segments?: Segment[]
}

const ImageViewer = ({ micrograph, segments }: ImageViewerProps) => {
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [viewerWidth, setViewerWidth] = useState<number>(0);
    const [viewerHeight, setViewerHeight] = useState<number>(0);
    const [showSegments, setShowSegments] = useState<boolean>(true);

    const viewerRef = useRef<HTMLImageElement>(null);

    const router = useRouter();

    useEffect(() => {
        // create event listener on windows resize
        window.addEventListener("resize", handleResize);

        // call handleResize to set initial state
        handleResize();

        // remove event listener on cleanup
        return () => window.removeEventListener("resize", handleResize);
    }, [])


    const handleResize = () => {
        if (viewerRef.current) {
            setViewerWidth(viewerRef.current.width);
            setViewerHeight(viewerRef.current.height);
        }
    }

    if (micrograph.status === "new") return <div>Image is not yet processed</div>

    return (
        <div className="space-y-4">
            <div className="bg-gray-900 relative">

                <TransformWrapper>
                    <TransformComponent>
                        <Image className={clsx(isLoading && "animate-pulse")} src={convertFileSrc(micrograph.display_path)} alt={micrograph.name} width={micrograph.width} height={micrograph.height} placeholder={"blur"} blurDataURL={"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII="} onLoadingComplete={() => setIsLoading(false)} ref={viewerRef} />
                        {!isLoading && segments && showSegments && segments.map(segment => {
                            const { location_x: x, location_y: y, width, height } = segment;

                            const positionX = Math.floor((x / micrograph.width) * viewerWidth);
                            const positionY = Math.floor((y / micrograph.height) * viewerHeight);

                            const widthX = Math.floor((width / micrograph.width) * viewerWidth);
                            const heightY = Math.floor((height / micrograph.height) * viewerHeight);

                            return <div onClick={() => router.push(`/micrographs/segments/${segment.uuid}`)} key={segment.uuid} style={{ left: positionX, top: positionY, width: widthX, height: heightY }} className={clsx("absolute border", segment.status == "ok" ? "border-green-600" : "border-red-600", "hover:border-blue-600 cursor-pointer")}></div>
                        })
                        }
                    </TransformComponent>
                </TransformWrapper>
            </div>
            <Toggle onValueChange={setShowSegments} value={showSegments}>
                <ToggleItem value={true} text="Zeige Segmente" icon={EyeIcon} />
                <ToggleItem value={false} text="Segmente verstecken" icon={EyeSlashIcon} />
            </Toggle>
        </div>
    );
};

export default ImageViewer;