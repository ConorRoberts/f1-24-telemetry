/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export interface paths {
    "/events": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: {
            parameters: {
                query?: never;
                header?: never;
                path?: never;
                cookie?: never;
            };
            requestBody?: never;
            responses: {
                200: {
                    headers: {
                        [name: string]: unknown;
                    };
                    content: {
                        "text/event-stream": components["schemas"]["Event"][];
                    };
                };
            };
        };
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        CarMotionEvent: {
            type: components["schemas"]["EventType"];
            /** Format: float */
            world_position_x: number;
            /** Format: float */
            world_position_y: number;
            /** Format: float */
            world_position_z: number;
            /** Format: float */
            g_force_lateral: number;
            /** Format: float */
            g_force_longitudinal: number;
            /** Format: float */
            g_force_vertical: number;
        };
        CarTelemetryEvent: {
            type: components["schemas"]["EventType"];
            /** Format: float */
            throttle: number;
            /** Format: float */
            brake: number;
            /** Format: uint16 */
            speed: number;
            brake_temp: number[];
            tyre_surface_temp: number[];
            tyre_inner_temp: number[];
            /** Format: uint16 */
            engine_temperature: number;
            tyre_pressure: number[];
        };
        Event: components["schemas"]["Event_CarTelemetryEvent"] | components["schemas"]["Event_CarMotionEvent"] | components["schemas"]["Event_LapDataEvent"] | components["schemas"]["Event_HeartbeatEvent"];
        /** @enum {string} */
        EventType: "car_telemetry" | "car_motion" | "lap_data" | "heartbeat";
        Event_CarMotionEvent: {
            /**
             * @example car_motion
             * @enum {string}
             */
            type: "car_motion";
        } & components["schemas"]["CarMotionEvent"] & {
            /**
             * @description discriminator enum property added by openapi-typescript
             * @enum {string}
             */
            type: "car_motion";
        };
        Event_CarTelemetryEvent: {
            /**
             * @example car_telemetry
             * @enum {string}
             */
            type: "car_telemetry";
        } & components["schemas"]["CarTelemetryEvent"] & {
            /**
             * @description discriminator enum property added by openapi-typescript
             * @enum {string}
             */
            type: "car_telemetry";
        };
        Event_HeartbeatEvent: {
            /**
             * @example heartbeat
             * @enum {string}
             */
            type: "heartbeat";
        } & components["schemas"]["HeartbeatEvent"] & {
            /**
             * @description discriminator enum property added by openapi-typescript
             * @enum {string}
             */
            type: "heartbeat";
        };
        Event_LapDataEvent: {
            /**
             * @example lap_data
             * @enum {string}
             */
            type: "lap_data";
        } & components["schemas"]["LapDataEvent"] & {
            /**
             * @description discriminator enum property added by openapi-typescript
             * @enum {string}
             */
            type: "lap_data";
        };
        HeartbeatEvent: {
            type: components["schemas"]["EventType"];
        };
        LapDataEvent: {
            type: components["schemas"]["EventType"];
            /** Format: uint32 */
            last_lap_time_in_ms: number;
            /** Format: uint32 */
            current_lap_time_in_ms: number;
            /** Format: uint16 */
            sector1_time_ms_part: number;
            /** Format: uint8 */
            sector1_time_minutes_part: number;
            /** Format: uint16 */
            sector2_time_ms_part: number;
            /** Format: uint8 */
            sector2_time_minutes_part: number;
            /** Format: uint16 */
            delta_to_car_in_front_ms_part: number;
            /** Format: uint8 */
            delta_to_car_in_front_minutes_part: number;
            /** Format: uint16 */
            delta_to_race_leader_ms_part: number;
            /** Format: uint8 */
            delta_to_race_leader_minutes_part: number;
            /** Format: float */
            lap_distance: number;
            /** Format: float */
            total_distance: number;
            /** Format: uint8 */
            car_position: number;
            /** Format: uint8 */
            current_lap_num: number;
            /** Format: uint8 */
            sector: number;
            /** Format: uint8 */
            current_lap_invalid: number;
            /** Format: uint8 */
            grid_position: number;
            /** Format: uint8 */
            driver_status: number;
            /** Format: uint8 */
            result_status: number;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export type operations = Record<string, never>;
