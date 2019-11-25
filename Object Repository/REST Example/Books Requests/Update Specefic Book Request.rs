<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update Specefic Book Request</description>
   <name>Update Specefic Book Request</name>
   <tag></tag>
   <elementGuidId>16349b5b-3ccd-4609-99f4-bd5b40a271b6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ID\&quot;: \&quot;${book_id}\&quot;,\n  \&quot;Title\&quot;: \&quot;${book_title}\&quot;,\n  \&quot;Description\&quot;: \&quot;${book_description}\&quot;,\n  \&quot;PageCount\&quot;: 0,\n  \&quot;Excerpt\&quot;: \&quot;${book_excerpt}\&quot;,\n  \&quot;PublishDate\&quot;: \&quot;2019-11-25T14:02:37.311Z\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://fakerestapi.azurewebsites.net/api/Books/1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>d0627feb-ccb0-4cef-9763-4155dcf12f9e</id>
      <masked>false</masked>
      <name>book_id</name>
   </variables>
   <variables>
      <defaultValue>'Updated Title'</defaultValue>
      <description></description>
      <id>1e066996-483c-458e-a413-1b1a96ac39da</id>
      <masked>false</masked>
      <name>book_title</name>
   </variables>
   <variables>
      <defaultValue>'Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. Updated Book Description text. '</defaultValue>
      <description></description>
      <id>840779e2-86a1-4c9e-87c6-cc5d8aa04e48</id>
      <masked>false</masked>
      <name>book_description</name>
   </variables>
   <variables>
      <defaultValue>'Updated Book Excerpt text. Updated Book Excerpt text. Updated Book Excerpt text. Updated Book Excerpt text. '</defaultValue>
      <description></description>
      <id>58799a23-43d9-4aa5-87d4-c0dcccd7cc8b</id>
      <masked>false</masked>
      <name>book_excerpt</name>
   </variables>
   <variables>
      <defaultValue>'&quot;2019-11-25T14:02:37.311Z&quot;'</defaultValue>
      <description></description>
      <id>1a6526eb-2690-4eba-94af-b51a7ecf7570</id>
      <masked>false</masked>
      <name>book_published</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
