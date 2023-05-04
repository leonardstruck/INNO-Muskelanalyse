import { useRouter } from "next/router";

const Default = () => {
    const { query } = useRouter();
    return (
        <div>Main Window: {query["id"]}</div>
    )
}

export default Default;