import { invoke } from "@tauri-apps/api/tauri"

const debug = () => {
    return (
        <div>
            <h1>Debug</h1>

            <button onClick={() => invoke("debug_segmentation")}>Test</button>            
        </div>
    )
}

export default debug