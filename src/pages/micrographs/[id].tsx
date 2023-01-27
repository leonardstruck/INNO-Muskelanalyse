import { useRouter } from 'next/router'

const MicrographPage = () => {
    const router = useRouter();
    const { id } = router.query;

    return <>
        <span onClick={() => router.back()}>Zurück</span>
        <h1>Micrograph {id}</h1>
    </>
}

export default MicrographPage