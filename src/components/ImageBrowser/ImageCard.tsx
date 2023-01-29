import type { Micrograph } from "../../../src-tauri/bindings/Micrograph"
import prettyBytes from "pretty-bytes";
import clsx from "clsx";
import { useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/tauri";

import { useRouter } from "next/router";
import Image from "next/image";

type ImageCardProps = {
    image: Micrograph
}

const ImageCard = ({ image }: ImageCardProps) => {
    const router = useRouter();
    const [imagePath, setImagePath] = useState<string | undefined>(undefined);
    useEffect(() => {
        if (image.thumbnail_path) {
            setImagePath(convertFileSrc(image.thumbnail_path));
        }
    }, [image.thumbnail_path])
    return (
        <li key={image.uuid} className="relative">
            <div className={clsx(imagePath == undefined && "animate-pulse", "group aspect-w-10 aspect-h-7 block w-full overflow-hidden rounded-lg bg-gray-300 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100")}>
                {imagePath && <Image alt="" className="pointer-events-none object-cover group-hover:opacity-75" src={imagePath} fill />}
                <button type="button" className="absolute inset-0 focus:outline-none" onClick={() => router.push(`/micrographs/${image.uuid}`)}>
                    <span className="sr-only">View details for {image.name}</span>
                </button>
            </div>
            <p className="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">{image.name}</p>
            <p className="pointer-events-none block text-sm font-medium text-gray-500">{image.file_size ? prettyBytes(image.file_size) : "wird importiert..."}</p>
        </li>
    );
}

export default ImageCard