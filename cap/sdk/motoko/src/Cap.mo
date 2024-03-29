/// Cap library
///
/// Minimal working example:
///
/// ```motoko
/// import Cap "mo:cap-motoko-library/Cap";
///
/// let cap = CapMotokoLibrary.Cap(?local_replica_router_id);
/// let token_contract_id = "rdmx6-jaaaa-aaaaa-aaadq-cai";
///
/// public func init() : async () {
///     // As a demo, the parameters are hard-typed here
///     // but could be declared in the function signature
///     // and pass when executing the request
///     let handshake = await cap.handshake(
///       local_replica_router_id,
///       token_contract_id,
///       creation_cycles
///     );
/// };
/// ```

import Result "mo:base/Result";
import Principal "mo:base/Principal";
import Debug "mo:base/Debug";
import Cycles "mo:base/ExperimentalCycles";
import Root "Root";
import Types "Types";
import Router "Router";
import IC "IC";
import Option "mo:base/Option";
import Prelude "mo:base/Prelude";

module {
    public class Cap(override_mainnet_router_id: ?Text) {
        let router_id = Option.get(override_mainnet_router_id, Router.mainnet_id);
        
        var rootBucket: ?Text = null;
        let ic: IC.ICActor = actor("aaaaa-aa");

        public func getTransaction(id: Nat64) : async Result.Result<Root.Event, Types.GetTransactionError> {
            let root = switch(rootBucket) {
                case(?r) { r };
                case(_) { Prelude.unreachable() };
            };
            let rb: Root.Self = actor(root);

            let transaction_response = await rb.get_transaction({ id=id; witness=false; });

            switch(transaction_response) {
                case (#Found(event, witness)) {
                    switch(event) {
                        case (null) {
                            #err(#invalidTransaction)
                        };
                        case (?event) {
                            #ok(event)
                        }
                    }
                };
                case (#Delegate(_, _)) {
                    #err(#unsupportedResponse)
                }
            }
        };

        public func insert(event: Root.IndefiniteEvent) : async Result.Result<Nat64, Types.InsertTransactionError> {
            let root = switch(rootBucket) {
                case(?r) { r };
                case(_) { Prelude.unreachable() };
            };
            let rb: Root.Self = actor(root);

            let insert_response = await rb.insert(event);

            // TODO: throw on error

            #ok(insert_response)
        };

        public func handshake(router_id : Text, token_contract_id : Text, creation_cycles: Nat): async () {
            let router: Router.Self = actor(router_id);

            let result = await router.get_token_contract_root_bucket({
                witness=false;
                canister=Principal.fromText(router_id);
            });

            switch(result.canister) {
                case(null) {
                    let settings: IC.CanisterSettings = {
                        controllers = ?[Principal.fromText(router_id)];
                        compute_allocation = null;
                        memory_allocation = null;
                        freezing_threshold = null;
                    };

                    let params: IC.CreateCanisterParams = {
                        settings = ?settings
                    };

                    // Add cycles and perform the create call
                    Cycles.add(creation_cycles);

                    let create_response = await ic.create_canister(params);

                    // Install the cap code
                    let canister = create_response.canister_id;

                    await router.install_bucket_code(canister);

                    // Find root by the token contract id
                    let result = await router.get_token_contract_root_bucket({
                        witness=false;
                        canister=Principal.fromText(token_contract_id);
                    });

                    switch(result.canister) {
                        case(null) {
                            assert(false);
                        };
                        case(?canister) {
                            rootBucket := ?Principal.toText(canister);
                        };
                    };
                };
                case (?canister) {
                    rootBucket := ?Principal.toText(canister);
                };
            };
        };
    };
}