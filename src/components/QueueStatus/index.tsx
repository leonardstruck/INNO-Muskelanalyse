import { invoke } from "@tauri-apps/api/tauri";
import { QueueInfo } from "../../../src-tauri/bindings/QueueInfo";
import { useQuery } from "@tanstack/react-query";
import Warmup from "./Warmup";
import { useAutoAnimate } from "@formkit/auto-animate/react";
import Idle from "./Idle";

const QueueStatus = () => {
    const { data } = useQuery(["queue_status"], fetchStatus, {
        onSuccess: (data) => {
            console.log(data)
        }
    })
    const [animationParent] = useAutoAnimate();


    return (
        <div ref={animationParent}>
            {data?.status === "Idle" && <Idle />}
            {data?.status === "Running" && null}
            {data?.status === "Paused" && null}
            {(data?.status === "Warmup" || data?.status === "Uninitialized") && <Warmup queueInfo={data} />}
        </div>
    )

}


const fetchStatus = async () => {
    return invoke("get_queue_info").then((res) => {
        return res as QueueInfo
    })
}

export default QueueStatus;