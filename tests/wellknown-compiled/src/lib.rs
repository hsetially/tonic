pub mod gen {
    pub mod google {
        pub mod protobuf {
            #![allow(clippy::doc_overindented_list_items)]
            tonic::include_proto!("google.protobuf");
        }
    }

    pub mod test {
        tonic::include_proto!("test");
    }
}

pub fn grok() {
    let _any = crate::gen::google::protobuf::Any {
        type_url: "foo".to_owned(),
        value: Vec::new(),
    };
}
