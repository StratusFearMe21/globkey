declare module "globkey" {
    /**
     * Loads and starts the globkey library
     */
    export function start(callback: (keypair: string[]) => void): void;
    /**
     * Unloads and stops the globkey library. Returns a string if the worker thread could not be stopped
     */
    export function unload(): void;
    /**
     * @returns a boolean value based on if globkey is runnig or not.
     * @remarks this method will only return `true` if `start()` has been called
     */
    export function isRunning(): boolean;
    /**
     * Unloads and stops the globkey library as well as kills the nodejs process. Returns a string if the worker thread could not be stopped
     */
    export function stop(): void;
    /**
     * Prints the current version of the library being used
     */
    export function version(): void;
}
