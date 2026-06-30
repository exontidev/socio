import "../../style.css";

function Input() {
  return (
    <>
      <div className="fixed bottom-0 w-full h-20">
        <form method="POST" className="flex justify-center">
          <div className="flex rounded-2xl shadow-hard border-2">
            <input
              type="text"
              name="message"
              placeholder="Your message..."
              className="pl-5 w-70 rounded-full border-none outline-none focus:outline-none focus:ring-0 caret-black"
            />

            <button
              type="button"
              className="w-10 flex justify-center items-center"
            >
              <div className="bg-amber-600 rounded-full border-2 w-5 h-5"></div>
            </button>
          </div>

          <div className="w-15 justify-center flex">
            <button
              type="submit"
              className="rounded-full shadow-hard border-2 w-12 h-12"
            >
              Send
            </button>
          </div>
        </form>
      </div>
    </>
  );
}

export default Input;
