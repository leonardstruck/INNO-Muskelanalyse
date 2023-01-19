import type { Micrograph } from "../../../src-tauri/bindings/Micrograph"
import prettyBytes from "pretty-bytes";

type ImageCardProps = {
    image: Micrograph
}

const ImageCard = ({ image }: ImageCardProps) => {
    return (
        <>
            <li key={image.uuid} className="relative">
                <div className="group aspect-w-10 aspect-h-7 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                    <img alt="" className="pointer-events-none object-cover group-hover:opacity-75" />
                    <button type="button" className="absolute inset-0 focus:outline-none">
                        <span className="sr-only">View details for {image.name}</span>
                    </button>
                </div>
                <p className="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">{image.name}</p>
                <p className="pointer-events-none block text-sm font-medium text-gray-500">{prettyBytes(image.file_size)}</p>
            </li>
        </>
    );
}

export default ImageCard