import { useEffect } from "react"
import { invoke } from "@tauri-apps/api"
import isClient from "../../lib/isClient"

const CasePage = () => {
    useEffect(() => {
        isClient && invoke("get_cases").then((res) => {
            console.log(res)
        })
    }, [])
    return <div>Case Page</div>
}

export default CasePage