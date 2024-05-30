import Image from 'next/image';

export function Header() {
    return (
        <header
            className="border-b flex flex-col items-center
                   justify-between sm:flex-row sm:px-4 h-12"
        >
            <span className="text-lg font-mono">CodeRunner3000</span>
            <h1 className="hidden sm:inline">Run code in browser</h1>
            <nav className="justify-between hidden sm:flex">
                <div>
                    <a className="m-1 underline" href="/code">
                        Code runner
                    </a>
                    <span className="m-1 underline text-gray-400 hover:cursor-not-allowed">
                        Gadgets
                    </span>
                </div>
                <a href="https://github.com/Gregofi/CodeRunner3000">
                    <Image
                        src="github-mark.svg"
                        alt="GitHub"
                        className="m-1"
                        width="24"
                        height="24"
                    />
                </a>
            </nav>
        </header>
    );
}
