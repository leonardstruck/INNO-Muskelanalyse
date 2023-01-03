import Sidebar from "./Sidebar";
import { navigation } from "./Sidebar";
import { useRouter } from "next/router";

type LayoutProps = {
    children: React.ReactNode;
}

const Layout = (props: LayoutProps) => {
    const { asPath } = useRouter();
    return (<div className="overscroll-none">
        <Sidebar />
        <div className="flex flex-1 flex-col md:pl-64 overflow-y-scroll">
            <main className="flex-1">
                <div className="py-6">
                    <div className="mx-auto max-w-7xl px-4 sm:px-6 md:px-8">
                        <h1 className="text-4xl font-display font-semibold text-gray-900 dark:text-gray-200">{navigation.find((item) => item.href == asPath)?.name ?? ""}</h1>
                    </div>
                    <div className="mx-auto max-w-7xl px-4 sm:px-6 md:px-8">
                        <div className="py-4">
                            {props.children}
                        </div>
                    </div>
                </div>
            </main>
        </div>
    </div>);
}

export default Layout;