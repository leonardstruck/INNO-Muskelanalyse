import type { NextPageWithLayout } from "../_app"
const Page: NextPageWithLayout = () => {
    return <>asdf</>
}

Page.getLayout = (page) => {
    return (
        <div className="h-full w-full">
            {page}
        </div>
    )
}

export default Page