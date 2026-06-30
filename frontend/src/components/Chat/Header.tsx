import "../../style.css";

function Header() {
  return (
    <>
      <header className=" flex items-center justify-between h-20 max-w-7xl">
        <div className="w-18 h-10">
          <button className="float-right flex flex-col items-center justify-center gap-1 w-12 h-12 rounded-full bg-white shadow-hard border-2">
            <span className="w-5 h-0.5 bg-black rounded"></span>
            <span className="w-5 h-0.5 bg-black rounded"></span>
            <span className="w-5 h-0.5 bg-black rounded"></span>
          </button>
        </div>

        <div className="flex items-center justify-center w-40 h-12 rounded-full bg-[#62C4DA] border-4 border-[#19839B] shadow-hard">
          <p className="font-syne text-2xl font-extrabold text-white">Global</p>
        </div>

        <div className="w-18 h-10">
          <button className="float-left w-12 h-12 rounded-full bg-white shadow-hard border-2"></button>
        </div>
      </header>
      <div className="bg-amber-400 h-1000 w-5"></div>
    </>
  );
}
export default Header;
