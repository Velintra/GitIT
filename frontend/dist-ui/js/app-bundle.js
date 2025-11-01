(function () {
    'use strict';

    // --------- Object Utils --------- //
    // return true if the value is null, undefined, empty array, empty string, or empty object
    // same as ensureMap but for array
    function ensureArray(obj, propName) {
        return _ensure(obj, propName, Array);
    }
    function _ensure(obj, propName, type) {
        const isMap = (obj instanceof Map);
        let v = (isMap) ? obj.get(propName) : obj[propName];
        if (v == null) {
            v = (type == null) ? {} : (type === Array) ? [] : (new type);
            if (isMap) {
                obj.set(propName, v);
            }
            else {
                obj[propName] = v;
            }
        }
        return v;
    }
    const emptyArray = Object.freeze([]);
    /**
     * Returns a readonly Node array from EventTarget, NodeList, Node[], or empty readonly array for null and undefined.
     */
    function asNodeArray(value) {
        if (value != null) {
            if (value instanceof Array) {
                return value;
            }
            // If it is a nodeList, copy the elements into a real array
            else if (value.constructor && value.constructor.name === "NodeList") {
                return Array.prototype.slice.call(value);
            }
            // FIXME: Needs to handle the document fragment case. 
            // otherwise we add value
            else {
                return [value]; // Note: here we assume it the evenTarget is a node
            }
        }
        // otherwise, return an empty array (readonly, so that we can )
        return emptyArray;
    }
    // --------- /asType --------- //
    // --------- String Utils --------- //
    function splitAndTrim(str, sep) {
        if (str == null) {
            return [];
        }
        if (str.indexOf(sep) === -1) {
            return [str.trim()];
        }
        return str.split(sep).map(trim);
    }
    function trim(str) {
        return str.trim();
    }

    //#endregion ---------- /Public off API ---------- 
    //#region    ---------- Public trigger API ---------- 
    const customDefaultProps = {
        bubbles: true,
        cancelable: true
    };
    function trigger(els, type, evtInit) {
        if (els == null) {
            return;
        } // for now make it null/undefined proof
        asNodeArray(els).forEach(function (el) {
            const evt = new CustomEvent(type, Object.assign({}, customDefaultProps, { selectTarget: el }, evtInit));
            el.dispatchEvent(evt);
        });
    }

    //#endregion ---------- /Private Helpers ---------- 
    //#region    ---------- Public Factory ---------- 
    /** Singleton hub factory */
    function hub(name) {
        if (name == null) {
            throw new Error('dom-native INVALID API CALLS: hub(name) require a name (no name was given).');
        }
        let hub = hubDic.get(name);
        // if it does not exist, we create and set it. 
        if (hub === undefined) {
            hub = new HubImpl(name);
            hubDic.set(name, hub);
            // create the hubData
            hubDataDic.set(name, new HubData(name));
        }
        return hub;
    }
    // User Hub object exposing the public API
    const hubDic = new Map();
    // Data for each hub (by name)
    const hubDataDic = new Map();
    class HubImpl {
        constructor(name) {
            this.name = name;
        }
        sub(topics, labels_or_handler, handler_or_opts, opts) {
            //// Build the arguments
            let labels;
            let handler;
            // if the first arg is function, then, no labels
            if (labels_or_handler instanceof Function) {
                labels = null;
                handler = labels_or_handler;
                opts = handler_or_opts;
            }
            else {
                labels = labels_or_handler;
                handler = handler_or_opts;
                // opts = opts;
            }
            //// Normalize topic and label to arrays
            const topicArray = splitAndTrim(topics, ",");
            const labelArray = (labels != null) ? splitAndTrim(labels, ",") : null;
            //// make opts (always defined at least an emtpy object)
            opts = makeOpts(opts);
            //// add the event to the hubData
            const hubData = hubDataDic.get(this.name); // by hub(...) factory function, this is garanteed
            hubData.addEvent(topicArray, labelArray, handler, opts);
        }
        unsub(ns) {
            const hubData = hubDataDic.get(this.name); // by factory contract, this always exist.
            hubData.removeRefsForNs(ns.ns);
        }
        pub(topics, labels, data) {
            // ARG SHIFTING: if data is undefined, we shift args to the RIGHT
            if (typeof data === "undefined") {
                data = labels;
                labels = null;
            }
            //// Normalize topic and label to arrays
            const topicArray = splitAndTrim(topics, ",");
            const labelArray = (labels != null) ? splitAndTrim(labels, ",") : null;
            const hubData = hubDataDic.get(this.name);
            const hasLabels = (labels != null && labels.length > 0);
            // if we have labels, then, we send the labels bound events first
            if (hasLabels) {
                hubData.getRefs(topicArray, labelArray).forEach(function (ref) {
                    invokeRef(ref, data);
                });
            }
            // then, we send the topic only bound
            hubData.getRefs(topicArray, null).forEach(function (ref) {
                // if this send, has label, then, we make sure we invoke for each of this label
                if (hasLabels) {
                    labelArray.forEach(function (label) {
                        invokeRef(ref, data, label);
                    });
                }
                // if we do not have labels, then, just call it.
                else {
                    invokeRef(ref, data);
                }
            });
        }
        deleteHub() {
            hubDic.delete(this.name);
            hubDataDic.delete(this.name);
        }
    }
    // TODO: This was maded to have it private to the hub. Now that we are using trypescript, we might want to use private and store it in the Hub. 
    class HubData {
        constructor(name) {
            this.refsByNs = new Map();
            this.refsByTopic = new Map();
            this.refsByTopicLabel = new Map();
            this.name = name;
        }
        addEvent(topics, labels, fun, opts) {
            const refs = buildRefs(topics, labels, fun, opts);
            const refsByNs = this.refsByNs;
            const refsByTopic = this.refsByTopic;
            const refsByTopicLabel = this.refsByTopicLabel;
            refs.forEach(function (ref) {
                // add this ref to the ns dictionary
                // TODO: probably need to add an custom "ns"
                if (ref.ns != null) {
                    ensureArray(refsByNs, ref.ns).push(ref);
                }
                // if we have a label, add this ref to the topicLabel dictionary
                if (ref.label != null) {
                    ensureArray(refsByTopicLabel, buildTopicLabelKey(ref.topic, ref.label)).push(ref);
                }
                // Otherwise, add it to this ref this topic
                else {
                    ensureArray(refsByTopic, ref.topic).push(ref);
                }
            });
        }
        ;
        getRefs(topics, labels) {
            const refs = [];
            const refsByTopic = this.refsByTopic;
            const refsByTopicLabel = this.refsByTopicLabel;
            topics.forEach(function (topic) {
                // if we do not have labels, then, just look in the topic dic
                if (labels == null || labels.length === 0) {
                    const topicRefs = refsByTopic.get(topic);
                    if (topicRefs) {
                        refs.push.apply(refs, topicRefs);
                    }
                }
                // if we have some labels, then, take those in accounts
                else {
                    labels.forEach(function (label) {
                        const topicLabelRefs = refsByTopicLabel.get(buildTopicLabelKey(topic, label));
                        if (topicLabelRefs) {
                            refs.push.apply(refs, topicLabelRefs);
                        }
                    });
                }
            });
            return refs;
        }
        ;
        removeRefsForNs(ns) {
            const refsByTopic = this.refsByTopic;
            const refsByTopicLabel = this.refsByTopicLabel;
            const refsByNs = this.refsByNs;
            const refs = this.refsByNs.get(ns);
            if (refs != null) {
                // we remove each ref from the corresponding dic
                refs.forEach(function (ref) {
                    // First, we get the refs from the topic or topiclabel
                    let refList;
                    if (ref.label != null) {
                        const topicLabelKey = buildTopicLabelKey(ref.topic, ref.label);
                        refList = refsByTopicLabel.get(topicLabelKey);
                    }
                    else {
                        refList = refsByTopic.get(ref.topic);
                    }
                    // Then, for the refList array, we remove the ones that match this object
                    let idx;
                    while ((idx = refList.indexOf(ref)) !== -1) {
                        refList.splice(idx, 1);
                    }
                });
                // we remove them all form the refsByNs
                refsByNs.delete(ns);
            }
        }
        ;
    }
    // static/private
    function buildRefs(topics, labels, fun, opts) {
        let refs = [];
        topics.forEach(function (topic) {
            // if we do not have any labels, then, just add this topic
            if (labels == null || labels.length === 0) {
                refs.push({
                    topic: topic,
                    fun: fun,
                    ns: opts.ns,
                    ctx: opts.ctx
                });
            }
            // if we have one or more labels, then, we add for those label
            else {
                labels.forEach(function (label) {
                    refs.push({
                        topic: topic,
                        label: label,
                        fun: fun,
                        ns: opts.ns,
                        ctx: opts.ctx
                    });
                });
            }
        });
        return refs;
    }
    // static/private: return a safe opts. If opts is a string, then, assume is it the {ns}
    const emptyOpts = {};
    function makeOpts(opts) {
        if (opts == null) {
            opts = emptyOpts;
        }
        else {
            if (typeof opts === "string") {
                opts = { ns: opts };
            }
        }
        return opts;
    }
    // static/private
    function buildTopicLabelKey(topic, label) {
        return topic + "-!-" + label;
    }
    // static/private: call ref method (with optional label override)
    function invokeRef(ref, data, label) {
        const info = {
            topic: ref.topic,
            label: ref.label || label,
            ns: ref.ns
        };
        ref.fun.call(ref.ctx, data, info);
    }
    //#endregion ---------- /Hub Implementation ----------

    document.createElement('div');
    document.createElement('e');

    /******************************************************************************
    Copyright (c) Microsoft Corporation.

    Permission to use, copy, modify, and/or distribute this software for any
    purpose with or without fee is hereby granted.

    THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
    REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
    AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
    INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
    LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
    OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
    PERFORMANCE OF THIS SOFTWARE.
    ***************************************************************************** */
    /* global Reflect, Promise, SuppressedError, Symbol, Iterator */


    typeof SuppressedError === "function" ? SuppressedError : function (error, suppressed, message) {
        var e = new Error(message);
        return e.name = "SuppressedError", e.error = error, e.suppressed = suppressed, e;
    };

    /**
     * Stores the callback in a known location, and returns an identifier that can be passed to the backend.
     * The backend uses the identifier to `eval()` the callback.
     *
     * @return An unique identifier associated with the callback function.
     *
     * @since 1.0.0
     */
    function transformCallback(
    // TODO: Make this not optional in v3
    callback, once = false) {
        return window.__TAURI_INTERNALS__.transformCallback(callback, once);
    }
    /**
     * Sends a message to the backend.
     * @example
     * ```typescript
     * import { invoke } from '@tauri-apps/api/core';
     * await invoke('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });
     * ```
     *
     * @param cmd The command name.
     * @param args The optional arguments to pass to the command.
     * @param options The request options.
     * @return A promise resolving or rejecting to the backend response.
     *
     * @since 1.0.0
     */
    async function invoke(cmd, args = {}, options) {
        return window.__TAURI_INTERNALS__.invoke(cmd, args, options);
    }

    // Copyright 2019-2024 Tauri Programme within The Commons Conservancy
    // SPDX-License-Identifier: Apache-2.0
    // SPDX-License-Identifier: MIT
    /**
     * The event system allows you to emit events to the backend and listen to events from it.
     *
     * This package is also accessible with `window.__TAURI__.event` when [`app.withGlobalTauri`](https://v2.tauri.app/reference/config/#withglobaltauri) in `tauri.conf.json` is set to `true`.
     * @module
     */
    /**
     * @since 1.1.0
     */
    var TauriEvent;
    (function (TauriEvent) {
        TauriEvent["WINDOW_RESIZED"] = "tauri://resize";
        TauriEvent["WINDOW_MOVED"] = "tauri://move";
        TauriEvent["WINDOW_CLOSE_REQUESTED"] = "tauri://close-requested";
        TauriEvent["WINDOW_DESTROYED"] = "tauri://destroyed";
        TauriEvent["WINDOW_FOCUS"] = "tauri://focus";
        TauriEvent["WINDOW_BLUR"] = "tauri://blur";
        TauriEvent["WINDOW_SCALE_FACTOR_CHANGED"] = "tauri://scale-change";
        TauriEvent["WINDOW_THEME_CHANGED"] = "tauri://theme-changed";
        TauriEvent["WINDOW_CREATED"] = "tauri://window-created";
        TauriEvent["WEBVIEW_CREATED"] = "tauri://webview-created";
        TauriEvent["DRAG_ENTER"] = "tauri://drag-enter";
        TauriEvent["DRAG_OVER"] = "tauri://drag-over";
        TauriEvent["DRAG_DROP"] = "tauri://drag-drop";
        TauriEvent["DRAG_LEAVE"] = "tauri://drag-leave";
    })(TauriEvent || (TauriEvent = {}));
    /**
     * Unregister the event listener associated with the given name and id.
     *
     * @ignore
     * @param event The event name
     * @param eventId Event identifier
     * @returns
     */
    async function _unlisten(event, eventId) {
        window.__TAURI_EVENT_PLUGIN_INTERNALS__.unregisterListener(event, eventId);
        await invoke('plugin:event|unlisten', {
            event,
            eventId
        });
    }
    /**
     * Listen to an emitted event to any {@link EventTarget|target}.
     *
     * @example
     * ```typescript
     * import { listen } from '@tauri-apps/api/event';
     * const unlisten = await listen<string>('error', (event) => {
     *   console.log(`Got error, payload: ${event.payload}`);
     * });
     *
     * // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
     * unlisten();
     * ```
     *
     * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
     * @param handler Event handler callback.
     * @param options Event listening options.
     * @returns A promise resolving to a function to unlisten to the event.
     * Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
     *
     * @since 1.0.0
     */
    async function listen(event, handler, options) {
        var _a;
        const target = ((_a = void 0 ) !== null && _a !== void 0 ? _a : { kind: 'Any' });
        return invoke('plugin:event|listen', {
            event,
            target,
            handler: transformCallback(handler)
        }).then((eventId) => {
            return async () => _unlisten(event, eventId);
        });
    }

    listen("HubEvent", function (evt) {
        const hubEvent = evt.payload;
        let _hub = hub(hubEvent.hub);
        if (hubEvent.label != null) {
            _hub.pub(hubEvent.topic, hubEvent.label, hubEvent.data);
        }
        else {
            _hub.pub(hubEvent.topic, hubEvent.data);
        }
    });

    document.addEventListener("DOMContentLoaded", async function (event) {
        trigger(this, "APP_LOADED");
        console.log("LOADED");
    });

})();
//# sourceMappingURL=app-bundle.js.map
