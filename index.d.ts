declare module "globkey" {
    /**
     * Loads and starts the globkey library
     */
    export function start(): void;
    /**
     * Unloads and stops the globkey library. Returns a string if the worker thread could not be stopped
     */
    export function unload(): void | string;
    /**
     * @returns an array of the currently pressed keys, or a string containing an error if it failed.
     * @remarks this method will only fucntion properly if the `start()` function has already been called
     */
    export function getKeys(): string[] | string;
    /**
     * @returns a boolean value based on if globkey is runnig or not.
     * @remarks this method will only return `true` if `start()` has been called
     */
    export function isRunning(): boolean;
    /**
     * Unloads and stops the globkey library as well as kills the nodejs process. Returns a string if the worker thread could not be stopped
     */
    export function stop(): void | string;
}