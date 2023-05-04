import Sidebar from './Sidebar'


type MainLayoutProps = {
    children: React.ReactNode
}

const MainLayout = ({ children }: MainLayoutProps) => {

    return (
        <>
            <div>
                <Sidebar />
                <main className="py-10 lg:pl-72">
                    <div className="px-4 sm:px-6 lg:px-8">{children}</div>
                </main>
            </div>
        </>
    )
}


export default MainLayout;