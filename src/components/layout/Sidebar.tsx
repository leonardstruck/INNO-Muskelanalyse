import clsx from "clsx";
import TechnikumLogo from "../../assets/technikum_logo.png"
import Image from "next/image";
import { HomeIcon, BeakerIcon } from "@heroicons/react/24/outline"

import { useRouter } from "next/router";
import Link from "next/link";

type NavigationItem = {
    name: string;
    href: string;
    icon: React.ComponentType<{ className: string }>;
}

export const navigation: NavigationItem[] = [
    {
        name: 'Dashboard',
        href: '/dashboard',
        icon: HomeIcon,
    },
    {
        name: 'Fälle',
        href: '/cases',
        icon: BeakerIcon,
    }
];

const Sidebar = () => {
    const { asPath, pathname } = useRouter();
    return (
        <div className="hidden md:fixed md:inset-y-0 md:flex md:w-64 md:flex-col">
            <div className="flex min-h-0 flex-1 flex-col border-r border-gray-200 bg-white dark:bg-gray-900 dark:border-gray-800">
                <div className="flex flex-1 flex-col overflow-y-auto pt-5">
                    <div className="flex flex-shrink-0 items-center px-4 space-x-4">
                        <div className="pl-2 w-20 dark:invert"><Image src={TechnikumLogo} alt="Technikum Logo" /></div>
                        <span className="dark:text-white font-display text-2xl">Muskel-Analyse</span>
                    </div>
                    <nav className="mt-5 flex-1 space-y-2 bg-white dark:bg-gray-800 px-2 py-4">
                        {navigation.map((item) => (
                            <Link
                                key={item.name}
                                href={item.href}
                                className={clsx(
                                    asPath.includes(item.href) ? 'bg-gray-100 text-gray-900' : 'text-gray-600 dark:text-gray-200 hover:bg-gray-50 hover:text-gray-900',
                                    'group flex items-center px-2 py-2 text-sm font-medium rounded-md'
                                )}
                            >
                                <item.icon
                                    className={clsx(
                                        asPath.includes(item.href) ? 'text-gray-500' : 'text-gray-400 group-hover:text-gray-500',
                                        'mr-3 flex-shrink-0 h-6 w-6'
                                    )}
                                    aria-hidden="true"
                                />
                                {item.name}
                            </Link>
                        ))}
                    </nav>
                </div>
            </div>
        </div>

    )
}

export default Sidebar