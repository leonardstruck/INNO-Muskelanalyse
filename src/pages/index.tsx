import { useRouter } from "next/router";
import { useEffect } from "react";

export default function Index() {
    const router = useRouter();
    useEffect(() => {
        if (router.pathname == "/")
            router.push("/dashboard");
    }, [router])
    return <></>;
};