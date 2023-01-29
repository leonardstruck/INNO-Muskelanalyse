import Sidebar from "./Sidebar";

type LayoutProps = {
    children: React.ReactNode;
}

const Layout = (props: LayoutProps) => {
    return (
        <div className="overscroll-none">
            <Sidebar />
            <div className="pl-64 overflow-y-scroll h-screen">
                <main>
                    <div className="py-6">
                        <div className="mx-auto px-4 sm:px-6 md:px-8">
                            <div className="py-4">
                                {props.children}
                            </div>
                        </div>
                    </div>
                </main>
            </div >
        </div >);
}

export default Layout;