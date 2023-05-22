import type { Component } from "solid-js";

export const NotFound: Component = () => {
  return (
    <>
      <h2 class="text-4xl text-indigo-700 font-black">Error 404: Not Found</h2>
      <button
        onclick={() => window.history.back()}
        class="mt-4 px-4 py-1 rounded-md bg-sky-600 hover:bg-sky-400"
      >
        &larr; Go back
      </button>
    </>
  );
};

/*
 * Onclick javascript to make browser go back to previous page?. VER:
 * https://stackoverflow.com/questions/8067510/onclick-javascript-to-make-browser-go-back-to-previous-page
 */
