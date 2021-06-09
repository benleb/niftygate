#[allow(dead_code)]
pub mod erc1155_holder_upgradeable {
  # [rustfmt :: skip] use ethcontract as ethcontract ;
  #[doc = "Generated by `ethcontract`"]
  #[derive(Clone)]
  pub struct Contract {
    methods: Methods,
  }
  impl Contract {
    #[doc = r" Retrieves the truffle artifact used to generate the type safe"]
    #[doc = r" API for this contract."]
    pub fn artifact() -> &'static self::ethcontract::Artifact {
      use self::ethcontract::private::lazy_static;
      use self::ethcontract::Artifact;
      lazy_static! {
        pub static ref ARTIFACT: Artifact = {
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"ERC1155HolderUpgradeable\",\n  \"sourceName\": \"contracts/token/ERC1155/utils/ERC1155HolderUpgradeable.sol\",\n  \"abi\": [\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"onERC1155BatchReceived\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"onERC1155Received\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"interfaceId\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"supportsInterface\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x608060405234801561001057600080fd5b506103b8806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806301ffc9a714610046578063bc197c811461006e578063f23a6e61146100a6575b600080fd5b61005961005436600461030c565b6100c5565b60405190151581526020015b60405180910390f35b61008d61007c366004610203565b63bc197c8160e01b95945050505050565b6040516001600160e01b03199091168152602001610065565b61008d6100b43660046102a9565b63f23a6e6160e01b95945050505050565b60006001600160e01b03198216630271189760e51b14806100f657506301ffc9a760e01b6001600160e01b03198316145b90505b919050565b80356001600160a01b03811681146100f957600080fd5b600082601f830112610125578081fd5b8135602067ffffffffffffffff8211156101415761014161036c565b8160051b61015082820161033b565b83815282810190868401838801850189101561016a578687fd5b8693505b8584101561018c57803583526001939093019291840191840161016e565b50979650505050505050565b600082601f8301126101a8578081fd5b813567ffffffffffffffff8111156101c2576101c261036c565b6101d5601f8201601f191660200161033b565b8181528460208386010111156101e9578283fd5b816020850160208301379081016020019190915292915050565b600080600080600060a0868803121561021a578081fd5b610223866100fe565b9450610231602087016100fe565b9350604086013567ffffffffffffffff8082111561024d578283fd5b61025989838a01610115565b9450606088013591508082111561026e578283fd5b61027a89838a01610115565b9350608088013591508082111561028f578283fd5b5061029c88828901610198565b9150509295509295909350565b600080600080600060a086880312156102c0578081fd5b6102c9866100fe565b94506102d7602087016100fe565b93506040860135925060608601359150608086013567ffffffffffffffff811115610300578182fd5b61029c88828901610198565b60006020828403121561031d578081fd5b81356001600160e01b031981168114610334578182fd5b9392505050565b604051601f8201601f1916810167ffffffffffffffff811182821017156103645761036461036c565b604052919050565b634e487b7160e01b600052604160045260246000fdfea264697066735822122052bf075811a7eb72fab7af74c850866d799978275f5d259d777b61fc78dd55e164736f6c63430008030033\",\n  \"deployedBytecode\": \"0x608060405234801561001057600080fd5b50600436106100415760003560e01c806301ffc9a714610046578063bc197c811461006e578063f23a6e61146100a6575b600080fd5b61005961005436600461030c565b6100c5565b60405190151581526020015b60405180910390f35b61008d61007c366004610203565b63bc197c8160e01b95945050505050565b6040516001600160e01b03199091168152602001610065565b61008d6100b43660046102a9565b63f23a6e6160e01b95945050505050565b60006001600160e01b03198216630271189760e51b14806100f657506301ffc9a760e01b6001600160e01b03198316145b90505b919050565b80356001600160a01b03811681146100f957600080fd5b600082601f830112610125578081fd5b8135602067ffffffffffffffff8211156101415761014161036c565b8160051b61015082820161033b565b83815282810190868401838801850189101561016a578687fd5b8693505b8584101561018c57803583526001939093019291840191840161016e565b50979650505050505050565b600082601f8301126101a8578081fd5b813567ffffffffffffffff8111156101c2576101c261036c565b6101d5601f8201601f191660200161033b565b8181528460208386010111156101e9578283fd5b816020850160208301379081016020019190915292915050565b600080600080600060a0868803121561021a578081fd5b610223866100fe565b9450610231602087016100fe565b9350604086013567ffffffffffffffff8082111561024d578283fd5b61025989838a01610115565b9450606088013591508082111561026e578283fd5b61027a89838a01610115565b9350608088013591508082111561028f578283fd5b5061029c88828901610198565b9150509295509295909350565b600080600080600060a086880312156102c0578081fd5b6102c9866100fe565b94506102d7602087016100fe565b93506040860135925060608601359150608086013567ffffffffffffffff811115610300578182fd5b61029c88828901610198565b60006020828403121561031d578081fd5b81356001600160e01b031981168114610334578182fd5b9392505050565b604051601f8201601f1916810167ffffffffffffffff811182821017156103645761036461036c565b604052919050565b634e487b7160e01b600052604160045260246000fdfea264697066735822122052bf075811a7eb72fab7af74c850866d799978275f5d259d777b61fc78dd55e164736f6c63430008030033\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
          artifact
        };
      }
      &ARTIFACT
    }
    #[doc = r" Creates a new contract instance with the specified `web3`"]
    #[doc = r" provider at the given `Address`."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching"]
    #[doc = r" `Abi` is actually deployed at the given address."]
    pub fn at<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      Contract::with_deployment_info(web3, address, None)
    }
    #[doc = r" Creates a new contract instance with the specified `web3` provider with"]
    #[doc = r" the given `Abi` at the given `Address` and an optional transaction hash."]
    #[doc = r" This hash is used to retrieve contract related information such as the"]
    #[doc = r" creation block (which is useful for fetching all historic events)."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching `Abi` is"]
    #[doc = r" actually deployed at the given address nor that the transaction hash,"]
    #[doc = r" when provided, is actually for this contract deployment."]
    pub fn with_deployment_info<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
      deployment_information: Option<ethcontract::common::DeploymentInformation>,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      use self::ethcontract::transport::DynTransport;
      use self::ethcontract::web3::api::Web3;
      use self::ethcontract::Instance;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let abi = Self::artifact().abi.clone();
      let instance = Instance::with_deployment_info(web3, abi, address, deployment_information);
      Contract::from_raw(instance)
    }
    #[doc = r" Creates a contract from a raw instance."]
    fn from_raw(instance: self::ethcontract::dyns::DynInstance) -> Self {
      let methods = Methods { instance };
      Contract { methods }
    }
    #[doc = r" Returns the contract address being used by this instance."]
    pub fn address(&self) -> self::ethcontract::Address {
      self.raw_instance().address()
    }
    #[doc = r" Returns the deployment information of the contract"]
    #[doc = r" if it is known, `None` otherwise."]
    pub fn deployment_information(&self) -> Option<ethcontract::common::DeploymentInformation> {
      self.raw_instance().deployment_information()
    }
    #[doc = r" Returns a reference to the default method options used by this"]
    #[doc = r" contract."]
    pub fn defaults(&self) -> &self::ethcontract::contract::MethodDefaults {
      &self.raw_instance().defaults
    }
    #[doc = r" Returns a mutable reference to the default method options used"]
    #[doc = r" by this contract."]
    pub fn defaults_mut(&mut self) -> &mut self::ethcontract::contract::MethodDefaults {
      &mut self.raw_instance_mut().defaults
    }
    #[doc = r" Returns a reference to the raw runtime instance used by this"]
    #[doc = r" contract."]
    pub fn raw_instance(&self) -> &self::ethcontract::dyns::DynInstance {
      &self.methods.instance
    }
    #[doc = r" Returns a mutable reference to the raw runtime instance used by"]
    #[doc = r" this contract."]
    fn raw_instance_mut(&mut self) -> &mut self::ethcontract::dyns::DynInstance {
      &mut self.methods.instance
    }
  }
  impl std::fmt::Debug for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.debug_tuple(stringify!(ERC1155HolderUpgradeable))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
    ) -> self::ethcontract::dyns::DynDeployBuilder<Self>
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      use self::ethcontract::contract::DeployBuilder;
      use self::ethcontract::dyns::DynTransport;
      use self::ethcontract::web3::api::Web3;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let bytecode = Self::artifact().bytecode.clone();
      DeployBuilder::new(web3, bytecode, ()).expect("valid deployment args")
    }
  }
  impl self::ethcontract::contract::Deploy<self::ethcontract::dyns::DynTransport> for Contract {
    type Context = self::ethcontract::common::Bytecode;
    fn bytecode(cx: &Self::Context) -> &self::ethcontract::common::Bytecode {
      cx
    }
    fn abi(_: &Self::Context) -> &self::ethcontract::common::Abi {
      &Self::artifact().abi
    }
    fn from_deployment(
      web3: self::ethcontract::dyns::DynWeb3,
      address: self::ethcontract::Address,
      transaction_hash: self::ethcontract::H256,
      _: Self::Context,
    ) -> Self {
      Self::with_deployment_info(&web3, address, Some(transaction_hash.into()))
    }
  }
  impl Contract {
    #[doc = r" Retrieves a reference to type containing all the generated"]
    #[doc = r" contract methods. This can be used for methods where the name"]
    #[doc = r" would collide with a common method (like `at` or `deployed`)."]
    pub fn methods(&self) -> &Methods {
      &self.methods
    }
  }
  #[doc = r" Type containing all contract methods for generated contract type."]
  #[derive(Clone)]
  pub struct Methods {
    instance: self::ethcontract::dyns::DynInstance,
  }
  #[allow(clippy::too_many_arguments, clippy::type_complexity)]
  impl Methods {
    #[doc = "Generated by `ethcontract`"]
    pub fn on_erc1155_batch_received(
      &self,
      p0: self::ethcontract::Address,
      p1: self::ethcontract::Address,
      p2: Vec<self::ethcontract::U256>,
      p3: Vec<self::ethcontract::U256>,
      p4: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 4]>> {
      self
        .instance
        .method([188, 25, 124, 129], (p0, p1, p2, p3, p4))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn on_erc1155_received(
      &self,
      p0: self::ethcontract::Address,
      p1: self::ethcontract::Address,
      p2: self::ethcontract::U256,
      p3: self::ethcontract::U256,
      p4: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 4]>> {
      self
        .instance
        .method([242, 58, 110, 97], (p0, p1, p2, p3, p4))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn supports_interface(
      &self,
      interface_id: self::ethcontract::tokens::Bytes<[u8; 4]>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([1, 255, 201, 167], (interface_id,))
        .expect("generated call")
    }
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
}
pub use self::erc1155_holder_upgradeable::Contract as ERC1155HolderUpgradeable;
